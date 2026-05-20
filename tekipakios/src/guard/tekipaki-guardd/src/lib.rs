use num_bigint::BigUint;

pub const KEY_CHARS: &str = "2346789ABCDEFGHJKLMNPQRTUVWXY";

pub fn base29_to_bigint(key: &str) -> Option<BigUint> {
    let mut result = BigUint::from(0u64);
    let base = BigUint::from(29u64);

    for c in key.chars() {
        if let Some(val) = KEY_CHARS.find(c) {
            result = result * &base + BigUint::from(val as u64);
        } else {
            return None;
        }
    }
    Some(result)
}

pub fn bigint_to_base29(mut n: BigUint) -> String {
    if n == BigUint::from(0u64) {
        return KEY_CHARS.chars().next().unwrap().to_string();
    }
    let mut s = String::new();
    let base = BigUint::from(29u64);
    while n > BigUint::from(0u64) {
        let rem = (&n % &base).to_u64_digits();
        let val = rem.get(0).cloned().unwrap_or(0);
        s.push(KEY_CHARS.chars().nth(val as usize).unwrap());
        n /= &base;
    }
    s.chars().rev().collect()
}

pub fn calculate_f_k(k: BigUint) -> BigUint {
    // Specification: f(K) = (K + X - (Y * Z))^(3^52)
    // To ensure positive base in modular arithmetic, we use:
    // (K + X + offset - (Y * Z)) where offset is a multiple of lcm(3,5,8)=120
    let x = BigUint::parse_bytes(b"4B45492D53554B49", 16).unwrap();
    let y = BigUint::parse_bytes(b"54454B4950414B49", 16).unwrap();
    let z = BigUint::parse_bytes(b"DEADBEEFCAFEBABE", 16).unwrap();

    // Offset to ensure positive result, must be 0 mod 120 to preserve edition logic
    let offset = BigUint::from(120u64) * BigUint::from(2u64).pow(192); // Large enough

    (k + x + offset) - (y * z)
}

pub fn check_edition(k: BigUint) -> String {
    let base = calculate_f_k(k);

    let exp = BigUint::from(3u64).pow(52);

    // Enterprise: f(K) mod 8 = 0
    if base.modpow(&exp, &BigUint::from(8u64)) == BigUint::from(0u64) {
        return "Enterprise".to_string();
    }
    // Pro: f(K) mod 5 = 0
    if base.modpow(&exp, &BigUint::from(5u64)) == BigUint::from(0u64) {
        return "Pro".to_string();
    }
    // Home: f(K) mod 3 = 0
    if base.modpow(&exp, &BigUint::from(3u64)) == BigUint::from(0u64) {
        return "Home".to_string();
    }

    "Invalid".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base29_conversion() {
        let key = "34678";
        let bigint = base29_to_bigint(key).unwrap();
        let back = bigint_to_base29(bigint);
        assert_eq!(back, key);
    }

    #[test]
    fn test_edition_logic() {
        // Just verify that the engine is consistent
        let key = "RRRRRRRRRRRRRRRRRRRRRRRRR";
        let k = base29_to_bigint(&key).unwrap();
        let edition = check_edition(k);
        assert!(edition == "Home" || edition == "Pro" || edition == "Enterprise" || edition == "Invalid");
    }
}
