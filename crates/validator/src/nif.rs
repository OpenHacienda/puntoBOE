/// Validate a Spanish NIF/NIE/CIF.
/// The input should be 9 characters, right-aligned with possible leading spaces.
pub fn validate_nif(nif: &str) -> bool {
    let nif = nif.trim();
    if nif.len() != 9 {
        return false;
    }

    let first = nif.chars().next().unwrap();
    let last = nif.chars().last().unwrap();

    match first {
        // NIE: X, Y, Z
        'X' | 'Y' | 'Z' => validate_nie(nif),
        // Special NIFs: K, L, M — must come before A..=W to avoid being caught by CIF arm
        'K' | 'L' | 'M' => validate_nif_special(nif),
        // CIF: starts with a letter A-W (excluding K, L, M handled above)
        'A'..='W' => validate_cif(nif),
        // Standard NIF: 8 digits + letter
        '0'..='9' => {
            if !last.is_ascii_alphabetic() {
                return false;
            }
            validate_nif_standard(nif)
        }
        _ => false,
    }
}

const NIF_LETTERS: &[u8] = b"TRWAGMYFPDXBNJZSQVHLCKE";

fn validate_nif_standard(nif: &str) -> bool {
    let digits = &nif[0..8];
    let letter = nif.chars().last().unwrap();

    if !digits.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    let num: u32 = digits.parse().unwrap_or(u32::MAX);
    if num == u32::MAX {
        return false;
    }

    let expected = NIF_LETTERS[(num % 23) as usize] as char;
    letter == expected
}

fn validate_nie(nif: &str) -> bool {
    let first = nif.chars().next().unwrap();
    let prefix = match first {
        'X' => '0',
        'Y' => '1',
        'Z' => '2',
        _ => return false,
    };

    let converted = format!("{}{}", prefix, &nif[1..]);
    validate_nif_standard(&converted)
}

fn validate_nif_special(nif: &str) -> bool {
    // K, L, M NIFs: treat the digit part like a standard NIF
    let digits = &nif[1..8];
    let letter = nif.chars().last().unwrap();

    if !digits.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    // For K, L, M: first char maps to 0, so number = 0DDDDDDD
    let num_str = format!("0{}", digits);
    let num: u32 = num_str.parse().unwrap_or(u32::MAX);
    if num == u32::MAX {
        return false;
    }

    let expected = NIF_LETTERS[(num % 23) as usize] as char;
    letter == expected
}

fn validate_cif(cif: &str) -> bool {
    let chars: Vec<char> = cif.chars().collect();
    let first = chars[0];
    let control = chars[8];

    // Digits in positions 2-8 (indices 1..8)
    let digits: Vec<u32> = chars[1..8]
        .iter()
        .map(|c| c.to_digit(10))
        .collect::<Option<Vec<_>>>()
        .unwrap_or_default();

    if digits.len() != 7 {
        return false;
    }

    // CIF algorithm: sum even-position digits + sum of doubled odd-position digits
    let mut sum = 0u32;

    // Odd positions (1-indexed: 1, 3, 5, 7) = indices 0, 2, 4, 6
    for &i in &[0usize, 2, 4, 6] {
        let doubled = digits[i] * 2;
        sum += doubled / 10 + doubled % 10;
    }

    // Even positions (1-indexed: 2, 4, 6) = indices 1, 3, 5
    for &i in &[1usize, 3, 5] {
        sum += digits[i];
    }

    let control_digit = (10 - (sum % 10)) % 10;
    let control_letter = (b'A' + control_digit as u8) as char;

    // Some CIF types use letter, others digit, some accept both
    match first {
        'A' | 'B' | 'E' | 'H' => {
            // Digit control
            control.to_digit(10) == Some(control_digit)
        }
        'P' | 'Q' | 'R' | 'S' | 'W' => {
            // Letter control
            control == control_letter
        }
        _ => {
            // Accept either
            control.to_digit(10) == Some(control_digit) || control == control_letter
        }
    }
}

#[cfg(test)]
mod test_nif {
    use super::*;

    #[test]
    fn test_valid_nif() {
        assert!(validate_nif("12345678Z"));
        assert!(validate_nif("00000000T"));
    }

    #[test]
    fn test_invalid_nif() {
        assert!(!validate_nif("12345678A"));
        assert!(!validate_nif("1234567"));
    }

    #[test]
    fn test_nie() {
        // X0000000T -> 00000000T
        assert!(validate_nif("X0000000T"));
        // Y0000000Z -> 10000000Z (10000000 % 23 = 14 -> 'Z')
        assert!(validate_nif("Y0000000Z"));
    }

    #[test]
    fn test_cif() {
        // A58818501
        assert!(validate_nif("A58818501"));
    }

    #[test]
    fn test_trimmed() {
        assert!(validate_nif(" 12345678Z"));
    }
}
