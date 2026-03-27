use crate::ValidationError;
use crate::parser::*;
use std::collections::HashMap;

pub fn validate_cross(
    t1: &Record,
    t2_records: &[&Record],
    errors: &mut Vec<ValidationError>,
    warnings: &mut Vec<ValidationError>,
) {
    let t1_raw = &t1.raw;

    // E107: TOTAL_REGISTROS_T2 (pos 136-144) must match actual count
    let total_t2_str = field(t1_raw, 136, 144);
    let total_t2_declared: usize = total_t2_str.trim().parse().unwrap_or(0);
    let total_t2_real = t2_records.len();
    if total_t2_declared != total_t2_real {
        errors.push(ValidationError::error_pos(
            "E107",
            t1.line,
            (136, 144),
            "TOTAL_REGISTROS_T2",
            &format!(
                "TOTAL_REGISTROS_T2 declarado ({}) != real ({})",
                total_t2_declared, total_t2_real
            ),
        ));
    }

    // E108: SUMA_VAL1 (pos 145-162, sign at 145) must match sum of T2 VAL1
    let sign_val1_t1 = field(t1_raw, 145, 145);
    let val1_t1_str = field(t1_raw, 146, 162);
    let val1_t1 = parse_signed_val(&sign_val1_t1, &val1_t1_str);

    let mut sum_val1: i64 = 0;
    for t2 in t2_records {
        let sign = field(&t2.raw, 432, 432);
        let val_str = field(&t2.raw, 433, 446);
        if is_numeric(val_str.trim()) {
            let v: i64 = val_str.trim().parse().unwrap_or(0);
            sum_val1 += if sign == "N" { -v } else { v };
        }
    }

    if val1_t1 != sum_val1 {
        errors.push(ValidationError::error_pos(
            "E108",
            t1.line,
            (145, 162),
            "SUMA_VAL1",
            &format!(
                "SUMA_VAL1 declarada ({}) != suma real ({})",
                val1_t1, sum_val1
            ),
        ));
    }

    // E109: SUMA_VAL2 (pos 163-180, sign at 163) must match sum of T2 VAL2
    let sign_val2_t1 = field(t1_raw, 163, 163);
    let val2_t1_str = field(t1_raw, 164, 180);
    let val2_t1 = parse_signed_val(&sign_val2_t1, &val2_t1_str);

    let mut sum_val2: i64 = 0;
    for t2 in t2_records {
        let sign = field(&t2.raw, 447, 447);
        let val_str = field(&t2.raw, 448, 461);
        if is_numeric(val_str.trim()) {
            let v: i64 = val_str.trim().parse().unwrap_or(0);
            sum_val2 += if sign == "N" { -v } else { v };
        }
    }

    if val2_t1 != sum_val2 {
        errors.push(ValidationError::error_pos(
            "E109",
            t1.line,
            (163, 180),
            "SUMA_VAL2",
            &format!(
                "SUMA_VAL2 declarada ({}) != suma real ({})",
                val2_t1, sum_val2
            ),
        ));
    }

    // W005: Multiple records with same ISIN and NIF_DECLARADO
    let mut seen: HashMap<(String, String), Vec<usize>> = HashMap::new();
    for t2 in t2_records {
        let clave_id = char_at(&t2.raw, 131);
        if clave_id == '1' {
            let isin = field(&t2.raw, 132, 143).trim().to_string();
            let nif_decl = field(&t2.raw, 18, 26).trim().to_string();
            if !isin.is_empty() {
                seen.entry((isin, nif_decl)).or_default().push(t2.line);
            }
        }
    }
    for ((isin, nif), lines) in &seen {
        if lines.len() > 1 {
            for &line in lines {
                warnings.push(ValidationError::warning(
                    "W005",
                    line,
                    "IDENTIFICACION_VALORES",
                    &format!(
                        "Múltiples registros con ISIN {} y NIF_DECLARADO {} (líneas {:?})",
                        isin, nif, lines
                    ),
                ));
            }
        }
    }
}

fn parse_signed_val(sign: &str, digits: &str) -> i64 {
    let v: i64 = digits.trim().parse().unwrap_or(0);
    if sign == "N" { -v } else { v }
}
