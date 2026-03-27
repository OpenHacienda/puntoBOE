/// A parsed record from a Modelo 720 file.
#[derive(Debug, Clone)]
pub struct Record {
    pub line: usize,
    pub tipo: char,
    pub modelo: String,
    pub ejercicio: String,
    pub raw: String,
}

/// Parse a single 500-char line into a Record.
pub fn parse_record(line_str: &str, line_num: usize) -> Record {
    let chars: Vec<char> = line_str.chars().collect();
    Record {
        line: line_num,
        tipo: chars[0],
        modelo: chars[1..4].iter().collect(),
        ejercicio: chars[4..8].iter().collect(),
        raw: line_str.to_string(),
    }
}

/// Extract a substring by 1-based positions (inclusive).
/// `field(raw, 9, 17)` returns characters at positions 9..=17.
pub fn field(raw: &str, start: usize, end: usize) -> String {
    let chars: Vec<char> = raw.chars().collect();
    if end > chars.len() || start < 1 {
        return String::new();
    }
    chars[start - 1..end].iter().collect()
}

/// Extract a single character at 1-based position.
pub fn char_at(raw: &str, pos: usize) -> char {
    raw.chars().nth(pos - 1).unwrap_or(' ')
}

/// Check if a string is all blanks (spaces).
pub fn is_blank(s: &str) -> bool {
    s.chars().all(|c| c == ' ')
}

/// Check if a string is all zeros.
pub fn is_zeros(s: &str) -> bool {
    s.chars().all(|c| c == '0')
}

/// Check if a string is all numeric digits.
pub fn is_numeric(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_digit())
}

/// Validate a date in AAAAMMDD format. Returns true if valid or "00000000".
pub fn validate_date(s: &str) -> bool {
    if s == "00000000" {
        return true;
    }
    if s.len() != 8 || !is_numeric(s) {
        return false;
    }
    let year: u32 = s[0..4].parse().unwrap_or(0);
    let month: u32 = s[4..6].parse().unwrap_or(0);
    let day: u32 = s[6..8].parse().unwrap_or(0);

    if month < 1 || month > 12 || day < 1 {
        return false;
    }

    let days_in_month = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => return false,
    };

    day <= days_in_month
}

/// Check if date is strictly "00000000" (empty date).
pub fn is_empty_date(s: &str) -> bool {
    s == "00000000"
}

fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

#[cfg(test)]
mod test_parser {
    use super::*;

    #[test]
    fn test_field_extraction() {
        let raw = "1720202412345678XNOMBRE".to_string() + &" ".repeat(477);
        assert_eq!(field(&raw, 1, 1), "1");
        assert_eq!(field(&raw, 2, 4), "720");
        assert_eq!(field(&raw, 5, 8), "2024");
    }

    #[test]
    fn test_validate_date() {
        assert!(validate_date("00000000"));
        assert!(validate_date("20240229")); // 2024 is leap
        assert!(!validate_date("20230229")); // 2023 is not
        assert!(validate_date("20231231"));
        assert!(!validate_date("20231301"));
        assert!(!validate_date("20231200"));
    }

    #[test]
    fn test_is_numeric() {
        assert!(is_numeric("12345"));
        assert!(!is_numeric("123a5"));
        assert!(!is_numeric(""));
    }
}
