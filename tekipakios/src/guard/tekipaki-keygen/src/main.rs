use num_bigint::BigUint;
use rand::Rng;
use guard_lib::{bigint_to_base29, check_edition};

fn main() {
    let mut rng = rand::thread_rng();
    let editions = vec!["Enterprise", "Pro", "Home"];

    for edition in editions {
        println!("Generating {} key...", edition);
        loop {
            // Generate random 120-bit number to have a good base
            let mut bytes = [0u8; 15];
            rng.fill(&mut bytes);
            let k = BigUint::from_bytes_be(&bytes);

            let current_edition = check_edition(k.clone());
            if current_edition == edition {
                let base29 = bigint_to_base29(k);
                // Ensure 25 chars, pad with 2 (the first char in KEY_CHARS) if needed
                let mut full_key = base29.clone();
                while full_key.len() < 25 {
                    full_key.insert(0, '2');
                }

                // Format with hyphens: XXXXX-XXXXX-XXXXX-XXXXX-XXXXX
                let formatted = format!("{}-{}-{}-{}-{}",
                    &full_key[0..5], &full_key[5..10], &full_key[10..15], &full_key[15..20], &full_key[20..25]);

                println!("Key: {}", formatted);
                break;
            }
        }
    }
}
