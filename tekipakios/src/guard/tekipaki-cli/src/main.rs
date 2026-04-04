use std::env;
use guard_lib::{base29_to_bigint, check_edition};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Tekipaki-CLI v1.6");
        println!("Usage:");
        println!("  activate-os <KEY>    Activate the OS with 25-char key (XXXXX-XXXXX-...)");
        println!("  get-edition          Show current edition");
        println!("  tasklist             List processes");
        println!("  ipconfig             Network configuration");
        println!("  cls                  Clear screen");
        return;
    }

    let cmd = &args[1];
    match cmd.as_str() {
        "activate-os" => {
            if args.len() < 3 {
                println!("Error: Product key is required (XXXXX-XXXXX-XXXXX-XXXXX-XXXXX)");
                return;
            }
            let key = &args[2];
            if key.len() != 29 || key.chars().filter(|&c| c == '-').count() != 4 {
                println!("Error: Invalid product key format. Hyphens are required.");
                return;
            }
            let raw_key = key.replace("-", "");
            if let Some(k) = base29_to_bigint(&raw_key) {
                let edition = check_edition(k);
                if edition != "Invalid" {
                    println!("Activation successful! Edition: {}", edition);
                    // In real OS, this would trigger edition unlock services
                } else {
                    println!("Error: Invalid product key.");
                }
            } else {
                println!("Error: Invalid characters in product key.");
            }
        },
        "get-edition" => {
            println!("Tekipaki OS v1.6 Professional (Evaluation Copy)");
        },
        "tasklist" => {
            let _ = std::process::Command::new("ps").arg("aux").status();
        },
        "ipconfig" => {
            let _ = std::process::Command::new("ip").arg("addr").status();
        },
        "cls" => {
            print!("\x1b[2J\x1b[1;1H");
        },
        _ => {
            println!("Unknown command: {}", cmd);
        }
    }
}
