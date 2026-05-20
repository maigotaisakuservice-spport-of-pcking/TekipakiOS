use std::io::{self, Write};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::thread;

mod death_penalty;

const LOCK_FILE: &str = "/var/lib/tekipaki/.lock";
const RETRY_FILE: &str = "/var/lib/tekipaki/retries";
const AUTH_LOG: &str = "/var/log/tekipaki/auth.log";
const PUBLIC_KEY: &str = "/etc/tekipaki/public.key";
const SIG_DIR: &str = "/etc/tekipaki/signatures";
const LICENSE_KEY_FILE: &str = "/etc/tekipaki/license.key";

fn verify_signatures() -> bool {
    println!("Checking component integrity...");
    if !std::path::Path::new(PUBLIC_KEY).exists() {
        println!("Warning: Public key not found. Skipping integrity check.");
        return true;
    }

    // List of binaries to verify
    let binaries = vec![
        "/usr/local/bin/tekipaki-settings",
        "/usr/local/bin/tekipaki-tre",
        "/usr/local/bin/tekipaki-taskmgr",
    ];

    for bin in binaries {
        let bin_path = std::path::Path::new(bin);
        if !bin_path.exists() { continue; }

        let bin_name = bin_path.file_name().unwrap().to_str().unwrap();
        let sig_path = format!("{}/{}.sig", SIG_DIR, bin_name);

        if !std::path::Path::new(&sig_path).exists() {
            println!("CRITICAL: Signature missing for {}!", bin_name);
            return false;
        }

        // Use openssl command to verify (simpler than linking ed25519-dalek for this agent's environment)
        let status = std::process::Command::new("openssl")
            .args(&[
                "pkeyutl", "-verify",
                "-pubin", "-inkey", PUBLIC_KEY, "-keyform", "DER",
                "-rawin", "-in", bin,
                "-sigfile", &sig_path
            ])
            .status();

        match status {
            Ok(s) if s.success() => {
                println!("Integrity verified: {}", bin_name);
            }
            _ => {
                println!("CRITICAL: Integrity failure detected in {}!", bin_name);
                return false;
            }
        }
    }
    true
}

fn get_retries() -> u32 {
    fs::read_to_string(RETRY_FILE)
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0)
}

fn increment_retries() -> u32 {
    let count = get_retries() + 1;
    let _ = fs::create_dir_all("/var/lib/tekipaki");
    let _ = fs::write(RETRY_FILE, count.to_string());
    count
}

fn reset_retries() {
    let _ = fs::write(RETRY_FILE, "0");
}

fn check_auth_history() -> bool {
    if let Ok(log) = fs::read_to_string(AUTH_LOG) {
        if log.contains("AUTH_SUCCESS") {
            return true;
        }
    }
    false
}

fn final_license_prompt() -> bool {
    println!("\x1b[1;33m[SAFETY CHECK]\x1b[0m Verification failed.");
    println!("Please enter a valid EULA key to prevent system termination:");

    let mut input = String::new();
    io::stdout().flush().unwrap();
    if io::stdin().read_line(&mut input).is_ok() {
        let key = input.trim().replace("-", "");
        if let Some(k) = guard_lib::base29_to_bigint(&key) {
            if guard_lib::check_edition(k) != "Invalid" {
                println!("Verification successful. System restored.");
                let _ = fs::create_dir_all("/var/log/tekipaki");
                let mut log = fs::OpenOptions::new().append(true).create(true).open(AUTH_LOG).unwrap();
                writeln!(log, "[{}] AUTH_SUCCESS: Manual recovery",
                    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()).unwrap();
                return true;
            }
        }
    }
    false
}

fn main() {
    println!("Tekipaki-Guard v1.6 Initializing...");

    // Verify component signatures
    if !verify_signatures() {
        println!("System integrity compromise detected!");

        let retries = increment_retries();
        if retries <= 20 {
            println!("Retry {}/20: System integrity failure. Please check your installation.", retries);
            // In a real GUI we would show a prompt, here we allow boot to continue but limited
        } else {
            // Final safety checks before Death Penalty
            println!("CRITICAL: Retry limit reached. Performing multi-stage safety check...");

            // 1. Re-verify license file if it exists
            let mut key_valid = false;
            if let Ok(key) = fs::read_to_string(LICENSE_KEY_FILE) {
                if let Some(k) = guard_lib::base29_to_bigint(&key.trim().replace("-", "")) {
                    if guard_lib::check_edition(k) != "Invalid" {
                        key_valid = true;
                    }
                }
            }

            // 2. Check auth history
            let auth_history = check_auth_history();

            if key_valid || auth_history {
                println!("Safety check: Valid credentials or history found. Delaying penalty.");
                reset_retries();
            } else {
                death_penalty::display_bsod();
                println!("\nFINAL CHANCE: Enter EULA key to abort disk wipe.");
                if !final_license_prompt() {
                    println!("Final verification failed.");
                    thread::sleep(Duration::from_secs(5));
                    death_penalty::perform_disk_wipe();
                    std::process::exit(1);
                }
                reset_retries();
            }
        }
    } else {
        reset_retries();
    }

    // Check for clock rollback
    if let Ok(last_time_str) = fs::read_to_string(LOCK_FILE) {
        if let Ok(last_time) = last_time_str.trim().parse::<u64>() {
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            if now < last_time {
                println!("System Error: Clock rollback detected!");
                death_penalty::display_bsod();
                thread::sleep(Duration::from_secs(3));
                death_penalty::perform_disk_wipe();
                std::process::exit(1);
            }
        }
    }

    // Save current time
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let _ = fs::create_dir_all("/var/lib/tekipaki");
    let _ = fs::write(LOCK_FILE, now.to_string());

    // License Check
    let key_file = "/etc/tekipaki/license.key";
    if let Ok(key) = fs::read_to_string(key_file) {
        let k_clean = key.trim().replace("-", "");
        if let Some(k_int) = guard_lib::base29_to_bigint(&k_clean) {
            let edition = guard_lib::check_edition(k_int);
            if edition == "Invalid" {
                println!("Invalid License Detected!");
                death_penalty::display_bsod();
                thread::sleep(Duration::from_secs(3));
                death_penalty::perform_disk_wipe();
            } else {
                println!("Tekipaki OS {} Edition Activated.", edition);
            }
        }
    } else {
        println!("No license found. Running in Trial Mode (30 mins)...");
        // Enforce trial limits here
    }

    println!("Daemon running in background...");
}
