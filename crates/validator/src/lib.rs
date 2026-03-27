mod cross;
mod nif;
mod parser;
mod tipo1;
mod tipo2;

pub use parser::Record;

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationError>,
    pub summary: Option<FileSummary>,
}

#[derive(Debug, Clone)]
pub struct FileSummary {
    pub ejercicio: String,
    pub nif_declarante: String,
    pub nombre_declarante: String,
    pub total_registros_t2_declarado: usize,
    pub total_registros_t2_real: usize,
    pub suma_val1: f64,
    pub suma_val2: f64,
    pub records: Vec<T2Detail>,
}

/// Key fields extracted from a single Tipo-2 record, for display purposes.
#[derive(Debug, Clone, PartialEq)]
pub struct T2Detail {
    pub line: usize,
    pub nif_declarado: String,
    pub nombre_declarado: String,
    /// C / V / I / S / B
    pub clave_bien: char,
    pub subclave: char,
    pub codigo_pais: String,
    pub fecha_incorporacion: String,
    /// A / M / C
    pub origen: char,
    pub valoracion1: f64,
    pub valoracion2: f64,
}

#[derive(Debug, Clone)]
pub struct ValidationError {
    pub code: String,
    pub line: usize,
    pub position: Option<(usize, usize)>,
    pub field: String,
    pub message: String,
    pub severity: Severity,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Severity {
    Fatal,
    Error,
    Warning,
}

impl ValidationError {
    fn fatal(code: &str, line: usize, field: &str, message: &str) -> Self {
        Self {
            code: code.to_string(),
            line,
            position: None,
            field: field.to_string(),
            message: message.to_string(),
            severity: Severity::Fatal,
        }
    }

    fn error_pos(code: &str, line: usize, pos: (usize, usize), field: &str, message: &str) -> Self {
        Self {
            code: code.to_string(),
            line,
            position: Some(pos),
            field: field.to_string(),
            message: message.to_string(),
            severity: Severity::Error,
        }
    }

    fn warning(code: &str, line: usize, field: &str, message: &str) -> Self {
        Self {
            code: code.to_string(),
            line,
            position: None,
            field: field.to_string(),
            message: message.to_string(),
            severity: Severity::Warning,
        }
    }
}

/// Main entry point — validates raw bytes of a Modelo 720 BOE file.
pub fn validate(bytes: &[u8]) -> ValidationResult {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // Decode ISO-8859-1 (Windows-1252 superset)
    let (text, encoding_used, had_errors) = encoding_rs::WINDOWS_1252.decode(bytes);
    if had_errors {
        errors.push(ValidationError::fatal(
            "E001",
            0,
            "FICHERO",
            "Codificación no es ISO-8859-1",
        ));
        return ValidationResult {
            is_valid: false,
            errors,
            warnings,
            summary: None,
        };
    }
    let _ = encoding_used;

    // Split into lines (handle CRLF)
    let raw_lines: Vec<&str> = if text.contains("\r\n") {
        text.split("\r\n").collect()
    } else {
        text.split('\n').collect()
    };

    // Remove trailing empty line from final CRLF
    let lines: Vec<&str> = raw_lines
        .iter()
        .copied()
        .filter(|l| !l.is_empty())
        .collect();

    if lines.is_empty() {
        errors.push(ValidationError::fatal(
            "E003",
            0,
            "FICHERO",
            "Fichero vacío: no se encontró registro tipo 1",
        ));
        return ValidationResult {
            is_valid: false,
            errors,
            warnings,
            summary: None,
        };
    }

    // Validate line lengths (E002)
    for (i, line) in lines.iter().enumerate() {
        let len = line.chars().count();
        if len != 500 {
            errors.push(ValidationError::fatal(
                "E002",
                i + 1,
                "REGISTRO",
                &format!("Longitud de línea incorrecta: {} (esperado 500)", len),
            ));
        }
    }

    // If any line has wrong length, we can't safely parse positions
    if errors.iter().any(|e| e.code == "E002") {
        return ValidationResult {
            is_valid: false,
            errors,
            warnings,
            summary: None,
        };
    }

    // Parse records
    let records: Vec<parser::Record> = lines
        .iter()
        .enumerate()
        .map(|(i, line)| parser::parse_record(line, i + 1))
        .collect();

    // E003: first record must be type 1
    if records[0].tipo != '1' {
        errors.push(ValidationError::fatal(
            "E003",
            1,
            "TIPO_REGISTRO",
            "Primer registro no es tipo 1",
        ));
        return ValidationResult {
            is_valid: false,
            errors,
            warnings,
            summary: None,
        };
    }

    // E004: only one type 1 record
    let t1_count = records.iter().filter(|r| r.tipo == '1').count();
    if t1_count > 1 {
        errors.push(ValidationError::fatal(
            "E004",
            1,
            "TIPO_REGISTRO",
            &format!("Más de un registro tipo 1 ({} encontrados)", t1_count),
        ));
    }

    // E005: unknown record types
    for r in &records {
        if r.tipo != '1' && r.tipo != '2' {
            errors.push(ValidationError::fatal(
                "E005",
                r.line,
                "TIPO_REGISTRO",
                &format!("Tipo de registro desconocido: '{}'", r.tipo),
            ));
        }
    }

    // E006: MODELO must be "720"
    for r in &records {
        if r.modelo != "720" {
            errors.push(ValidationError::fatal(
                "E006",
                r.line,
                "MODELO",
                &format!("MODELO no es '720': '{}'", r.modelo),
            ));
        }
    }

    // E007: EJERCICIO
    for r in &records {
        if !r.ejercicio.chars().all(|c| c.is_ascii_digit()) || r.ejercicio.len() != 4 {
            errors.push(ValidationError::fatal(
                "E007",
                r.line,
                "EJERCICIO",
                &format!(
                    "EJERCICIO no numérico o longitud incorrecta: '{}'",
                    r.ejercicio
                ),
            ));
        } else {
            let year: u32 = r.ejercicio.parse().unwrap_or(0);
            if year < 1900 || year > 2100 {
                errors.push(ValidationError::fatal(
                    "E007",
                    r.line,
                    "EJERCICIO",
                    &format!("EJERCICIO fuera de rango: {}", year),
                ));
            }
        }
    }

    if errors.iter().any(|e| e.severity == Severity::Fatal) {
        return ValidationResult {
            is_valid: false,
            errors,
            warnings,
            summary: None,
        };
    }

    // Validate tipo 1
    let t1 = &records[0];
    tipo1::validate_tipo1(t1, &mut errors);

    // Validate tipo 2 records
    let t2_records: Vec<&parser::Record> = records.iter().filter(|r| r.tipo == '2').collect();
    for t2 in &t2_records {
        tipo2::validate_tipo2(t2, t1, &mut errors, &mut warnings);
    }

    // Cross-validations
    cross::validate_cross(t1, &t2_records, &mut errors, &mut warnings);

    // Build summary — use parser::field() (char-based) throughout; raw byte slices
    // are wrong whenever the nombre contains accented UTF-8 characters (á, é, ñ, …).
    let total_t2_declarado = parser::field(&t1.raw, 136, 144)
        .trim()
        .parse::<usize>()
        .unwrap_or(0);
    let suma_val1_t1 = parse_signed_amount(
        &parser::field(&t1.raw, 145, 145),
        &parser::field(&t1.raw, 146, 162),
    );
    let suma_val2_t1 = parse_signed_amount(
        &parser::field(&t1.raw, 163, 163),
        &parser::field(&t1.raw, 164, 180),
    );

    let records: Vec<T2Detail> = t2_records
        .iter()
        .map(|r| {
            let raw = &r.raw;
            T2Detail {
                line: r.line,
                nif_declarado: parser::field(raw, 18, 26).trim().to_string(),
                nombre_declarado: parser::field(raw, 36, 75).trim().to_string(),
                clave_bien: parser::char_at(raw, 102),
                subclave: parser::char_at(raw, 103),
                codigo_pais: parser::field(raw, 129, 130),
                fecha_incorporacion: parser::field(raw, 415, 422),
                origen: parser::char_at(raw, 423),
                valoracion1: parse_signed_amount(
                    &parser::field(raw, 432, 432),
                    &parser::field(raw, 433, 446),
                ),
                valoracion2: parse_signed_amount(
                    &parser::field(raw, 447, 447),
                    &parser::field(raw, 448, 461),
                ),
            }
        })
        .collect();

    let summary = FileSummary {
        ejercicio: t1.ejercicio.clone(),
        nif_declarante: parser::field(&t1.raw, 9, 17).trim().to_string(),
        nombre_declarante: parser::field(&t1.raw, 18, 57).trim().to_string(),
        total_registros_t2_declarado: total_t2_declarado,
        total_registros_t2_real: t2_records.len(),
        suma_val1: suma_val1_t1,
        suma_val2: suma_val2_t1,
        records,
    };

    let is_valid = errors.is_empty();
    ValidationResult {
        is_valid,
        errors,
        warnings,
        summary: Some(summary),
    }
}

fn parse_signed_amount(sign: &str, digits: &str) -> f64 {
    let val = digits.trim().parse::<i64>().unwrap_or(0) as f64 / 100.0;
    if sign == "N" { -val } else { val }
}

#[cfg(test)]
mod tests;
