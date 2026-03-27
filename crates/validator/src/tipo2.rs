use crate::ValidationError;
use crate::nif;
use crate::parser::*;

/// ISO-3166-1 alpha-2 country codes (subset for validation).
fn is_valid_country_code(code: &str) -> bool {
    // Accept any two uppercase ASCII letters as a basic check.
    // A full list could be used, but this covers the format requirement.
    let chars: Vec<char> = code.chars().collect();
    chars.len() == 2 && chars[0].is_ascii_uppercase() && chars[1].is_ascii_uppercase()
}

/// Validate ISIN: 2 letters + 9 alphanumeric + 1 check digit (Luhn).
fn validate_isin(isin: &str) -> bool {
    let isin = isin.trim();
    if isin.len() != 12 {
        return false;
    }
    let chars: Vec<char> = isin.chars().collect();
    if !chars[0].is_ascii_uppercase() || !chars[1].is_ascii_uppercase() {
        return false;
    }
    if !chars[2..11].iter().all(|c| c.is_ascii_alphanumeric()) {
        return false;
    }
    if !chars[11].is_ascii_digit() {
        return false;
    }

    // Luhn check on the expanded digits
    let mut digits = Vec::new();
    for c in &chars {
        if c.is_ascii_digit() {
            digits.push(*c as u32 - '0' as u32);
        } else if c.is_ascii_uppercase() {
            let val = *c as u32 - 'A' as u32 + 10;
            digits.push(val / 10);
            digits.push(val % 10);
        } else {
            return false;
        }
    }

    // Standard Luhn
    let mut sum = 0u32;
    let len = digits.len();
    for (i, &d) in digits.iter().enumerate() {
        let pos_from_right = len - 1 - i;
        if pos_from_right % 2 == 1 {
            let doubled = d * 2;
            sum += if doubled > 9 { doubled - 9 } else { doubled };
        } else {
            sum += d;
        }
    }
    sum % 10 == 0
}

pub fn validate_tipo2(
    record: &Record,
    t1: &Record,
    errors: &mut Vec<ValidationError>,
    warnings: &mut Vec<ValidationError>,
) {
    let raw = &record.raw;
    let line = record.line;

    // E233: EJERCICIO in T2 must match T1
    if record.ejercicio != t1.ejercicio {
        errors.push(ValidationError::error_pos(
            "E233",
            line,
            (5, 8),
            "EJERCICIO",
            &format!(
                "EJERCICIO en T2 ({}) != T1 ({})",
                record.ejercicio, t1.ejercicio
            ),
        ));
    }

    // E201: NIF_DECLARANTE (pos 9-17) must match T1
    let nif_decl_t2 = field(raw, 9, 17);
    let nif_decl_t1 = field(&t1.raw, 9, 17);
    if nif_decl_t2 != nif_decl_t1 {
        errors.push(ValidationError::error_pos(
            "E201",
            line,
            (9, 17),
            "NIF_DECLARANTE",
            &format!(
                "NIF_DECLARANTE en T2 ({}) != T1 ({})",
                nif_decl_t2.trim(),
                nif_decl_t1.trim()
            ),
        ));
    }

    // E202: NIF_DECLARADO (pos 18-26) must be valid
    let nif_declarado = field(raw, 18, 26);
    if !nif::validate_nif(&nif_declarado) {
        errors.push(ValidationError::error_pos(
            "E202",
            line,
            (18, 26),
            "NIF_DECLARADO",
            &format!("NIF_DECLARADO inválido: '{}'", nif_declarado.trim()),
        ));
    }

    // E203: APELLIDOS_NOMBRE_DECLARADO (pos 36-75) must not be empty
    let nombre = field(raw, 36, 75);
    if nombre.trim().is_empty() {
        errors.push(ValidationError::error_pos(
            "E203",
            line,
            (36, 75),
            "APELLIDOS_NOMBRE_DECLARADO",
            "APELLIDOS_NOMBRE_DECLARADO vacío",
        ));
    }

    // E204: CLAVE_CONDICION (pos 76) must be 1-8
    let clave_cond = char_at(raw, 76);
    let clave_cond_num = clave_cond.to_digit(10);
    if clave_cond_num.is_none() || clave_cond_num.unwrap() < 1 || clave_cond_num.unwrap() > 8 {
        errors.push(ValidationError::error_pos(
            "E204",
            line,
            (76, 76),
            "CLAVE_CONDICION",
            &format!("CLAVE_CONDICION fuera de rango: '{}'", clave_cond),
        ));
    }

    // E205: TIPO_TITULARIDAD (pos 77-101) must be blank when condition != 8
    let tipo_tit = field(raw, 77, 101);
    if clave_cond != '8' && !is_blank(&tipo_tit) {
        errors.push(ValidationError::error_pos(
            "E205",
            line,
            (77, 101),
            "TIPO_TITULARIDAD",
            "TIPO_TITULARIDAD no vacío cuando condición != 8",
        ));
    }

    // E206: CLAVE_TIPO_BIEN (pos 102) must be C, V, I, S, B
    let clave_bien = char_at(raw, 102);
    let valid_bien = matches!(clave_bien, 'C' | 'V' | 'I' | 'S' | 'B');
    if !valid_bien {
        errors.push(ValidationError::error_pos(
            "E206",
            line,
            (102, 102),
            "CLAVE_TIPO_BIEN",
            &format!("CLAVE_TIPO_BIEN inválida: '{}'", clave_bien),
        ));
    }

    // E207: SUBCLAVE (pos 103) valid for CLAVE_TIPO_BIEN
    let subclave = char_at(raw, 103);
    if valid_bien {
        let valid_subclave = match clave_bien {
            'C' => matches!(subclave, '1' | '2' | '3' | '4' | '5'),
            'V' => matches!(subclave, '1' | '2' | '3'),
            'I' => subclave == '0',
            'S' => matches!(subclave, '1' | '2'),
            'B' => matches!(subclave, '1' | '2' | '3' | '4' | '5'),
            _ => true,
        };
        if !valid_subclave {
            errors.push(ValidationError::error_pos(
                "E207",
                line,
                (103, 103),
                "SUBCLAVE",
                &format!(
                    "SUBCLAVE '{}' inválida para CLAVE_TIPO_BIEN '{}'",
                    subclave, clave_bien
                ),
            ));
        }
    }

    // E208: SUBCLAVE must be '0' when CLAVE_TIPO_BIEN = 'I'
    if clave_bien == 'I' && subclave != '0' {
        errors.push(ValidationError::error_pos(
            "E208",
            line,
            (103, 103),
            "SUBCLAVE",
            &format!(
                "SUBCLAVE debe ser '0' para CLAVE_TIPO_BIEN 'I', encontrado '{}'",
                subclave
            ),
        ));
    }

    // E209: CODIGO_PAIS (pos 129-130) must be ISO-3166
    let codigo_pais = field(raw, 129, 130);
    if !is_valid_country_code(&codigo_pais) {
        errors.push(ValidationError::error_pos(
            "E209",
            line,
            (129, 130),
            "CODIGO_PAIS",
            &format!("CODIGO_PAIS no es ISO-3166: '{}'", codigo_pais),
        ));
    }

    // E210: CLAVE_IDENTIFICACION (pos 131) must be 0, 1, or 2
    let clave_id = char_at(raw, 131);
    if !matches!(clave_id, '0' | '1' | '2') {
        errors.push(ValidationError::error_pos(
            "E210",
            line,
            (131, 131),
            "CLAVE_IDENTIFICACION",
            &format!("CLAVE_IDENTIFICACION inválida: '{}'", clave_id),
        ));
    }

    // E211: CLAVE_IDENTIFICACION must be 0 when bien is C, S, B
    if matches!(clave_bien, 'C' | 'S' | 'B') && clave_id != '0' {
        errors.push(ValidationError::error_pos(
            "E211",
            line,
            (131, 131),
            "CLAVE_IDENTIFICACION",
            &format!(
                "CLAVE_IDENTIFICACION debe ser '0' para bien '{}', encontrado '{}'",
                clave_bien, clave_id
            ),
        ));
    }

    // E212: ISIN validation when CLAVE_IDENTIFICACION = 1
    let id_valores = field(raw, 132, 143);
    if clave_id == '1' && !validate_isin(&id_valores) {
        errors.push(ValidationError::error_pos(
            "E212",
            line,
            (132, 143),
            "IDENTIFICACION_VALORES",
            &format!("ISIN inválido: '{}'", id_valores.trim()),
        ));
    }

    // E213: When CLAVE_IDENTIFICACION = 2, must not start with 'Z' — actually spec says
    // "No empieza por 'Z'" but this seems to mean it should NOT start with Z (likely a code
    // that starts with Z is reserved for ISINs). Let's implement as the spec says.
    if clave_id == '2' && id_valores.starts_with('Z') {
        errors.push(ValidationError::error_pos(
            "E213",
            line,
            (132, 143),
            "IDENTIFICACION_VALORES",
            "IDENTIFICACION_VALORES empieza por 'Z' cuando CLAVE_IDENTIFICACION = 2",
        ));
    }

    // E214: IDENTIFICACION_VALORES must be blank for non V/I
    if !matches!(clave_bien, 'V' | 'I') && !is_blank(&id_valores) {
        errors.push(ValidationError::error_pos(
            "E214",
            line,
            (132, 143),
            "IDENTIFICACION_VALORES",
            "IDENTIFICACION_VALORES no en blanco para bien que no es V/I",
        ));
    }

    // E215: CLAVE_ID_CUENTA (pos 144) must be 'I' or 'O' for bien C
    let clave_id_cuenta = char_at(raw, 144);
    if clave_bien == 'C' && !matches!(clave_id_cuenta, 'I' | 'O') {
        errors.push(ValidationError::error_pos(
            "E215",
            line,
            (144, 144),
            "CLAVE_ID_CUENTA",
            &format!(
                "CLAVE_ID_CUENTA inválida para bien C: '{}'",
                clave_id_cuenta
            ),
        ));
    }

    // E216: IDENTIFICACION_ENTIDAD (pos 190-230) must not be empty (except for B)
    let id_entidad = field(raw, 190, 230);
    if clave_bien != 'B' && id_entidad.trim().is_empty() {
        errors.push(ValidationError::error_pos(
            "E216",
            line,
            (190, 230),
            "IDENTIFICACION_ENTIDAD",
            "IDENTIFICACION_ENTIDAD vacía (obligatoria salvo para bien B)",
        ));
    }

    // E217/E218: FECHA_INCORPORACION (pos 415-422)
    let fecha_inc = field(raw, 415, 422);
    if !is_numeric(&fecha_inc) {
        errors.push(ValidationError::error_pos(
            "E217",
            line,
            (415, 422),
            "FECHA_INCORPORACION",
            &format!("FECHA_INCORPORACION formato incorrecto: '{}'", fecha_inc),
        ));
    } else if !validate_date(&fecha_inc) {
        errors.push(ValidationError::error_pos(
            "E218",
            line,
            (415, 422),
            "FECHA_INCORPORACION",
            &format!("FECHA_INCORPORACION fecha inválida: '{}'", fecha_inc),
        ));
    }

    // E219: FECHA_INCORPORACION mandatory for C
    if clave_bien == 'C' && is_empty_date(&fecha_inc) {
        errors.push(ValidationError::error_pos(
            "E219",
            line,
            (415, 422),
            "FECHA_INCORPORACION",
            "FECHA_INCORPORACION obligatoria para bien C pero es ceros",
        ));
    }

    // E220: ORIGEN (pos 423) must be A, M, or C
    let origen = char_at(raw, 423);
    if !matches!(origen, 'A' | 'M' | 'C') {
        errors.push(ValidationError::error_pos(
            "E220",
            line,
            (423, 423),
            "ORIGEN",
            &format!("ORIGEN debe ser A/M/C, encontrado '{}'", origen),
        ));
    }

    // E221/E222/E223: FECHA_EXTINCION (pos 424-431)
    let fecha_ext = field(raw, 424, 431);
    if origen == 'C' && is_empty_date(&fecha_ext) {
        errors.push(ValidationError::error_pos(
            "E221",
            line,
            (424, 431),
            "FECHA_EXTINCION",
            "FECHA_EXTINCION es ceros cuando ORIGEN=C",
        ));
    }
    if origen != 'C' && !is_empty_date(&fecha_ext) {
        errors.push(ValidationError::error_pos(
            "E222",
            line,
            (424, 431),
            "FECHA_EXTINCION",
            "FECHA_EXTINCION no es ceros cuando ORIGEN!=C",
        ));
    }
    if !is_empty_date(&fecha_ext) {
        if !is_numeric(&fecha_ext) || !validate_date(&fecha_ext) {
            errors.push(ValidationError::error_pos(
                "E223",
                line,
                (424, 431),
                "FECHA_EXTINCION",
                &format!("FECHA_EXTINCION inválida: '{}'", fecha_ext),
            ));
        }
    }

    // E224: VALORACION1 (pos 433-446) must be numeric
    let val1 = field(raw, 433, 446);
    if !is_numeric(val1.trim()) && !is_blank(&val1) {
        errors.push(ValidationError::error_pos(
            "E224",
            line,
            (433, 446),
            "VALORACION1",
            &format!("VALORACION1 no numérica: '{}'", val1.trim()),
        ));
    }

    // E226: CLAVE_REPRESENT_VALORES (pos 462) must be A or B for V/I
    let clave_repr = char_at(raw, 462);
    if matches!(clave_bien, 'V' | 'I') && !matches!(clave_repr, 'A' | 'B') {
        errors.push(ValidationError::error_pos(
            "E226",
            line,
            (462, 462),
            "CLAVE_REPRESENT_VALORES",
            &format!(
                "CLAVE_REPRESENT_VALORES inválida para V/I: '{}'",
                clave_repr
            ),
        ));
    }

    // E227: CLAVE_REPRESENT_VALORES must be blank for non V/I
    if !matches!(clave_bien, 'V' | 'I') && clave_repr != ' ' {
        errors.push(ValidationError::error_pos(
            "E227",
            line,
            (462, 462),
            "CLAVE_REPRESENT_VALORES",
            "CLAVE_REPRESENT_VALORES no en blanco para bien que no es V/I",
        ));
    }

    // E228: NUM_VALORES (pos 463-474) must be numeric for V/I
    let num_val = field(raw, 463, 474);
    if matches!(clave_bien, 'V' | 'I') && !is_numeric(&num_val) {
        errors.push(ValidationError::error_pos(
            "E228",
            line,
            (463, 474),
            "NUM_VALORES",
            &format!("NUM_VALORES no numérico para V/I: '{}'", num_val.trim()),
        ));
    }

    // E229: NUM_VALORES must be zeros for non V/I
    if !matches!(clave_bien, 'V' | 'I') && !is_zeros(&num_val) && !is_blank(&num_val) {
        errors.push(ValidationError::error_pos(
            "E229",
            line,
            (463, 474),
            "NUM_VALORES",
            "NUM_VALORES no es ceros para bien que no es V/I",
        ));
    }

    // E230: CLAVE_TIPO_INMUEBLE (pos 475) must be U or R for B
    let clave_inm = char_at(raw, 475);
    if clave_bien == 'B' && !matches!(clave_inm, 'U' | 'R') {
        errors.push(ValidationError::error_pos(
            "E230",
            line,
            (475, 475),
            "CLAVE_TIPO_INMUEBLE",
            &format!("CLAVE_TIPO_INMUEBLE inválida para B: '{}'", clave_inm),
        ));
    }

    // E231: PORCENTAJE_PARTICIPACION (pos 476-480) range check
    let pct_str = field(raw, 476, 480);
    if is_numeric(&pct_str) {
        let pct: u32 = pct_str.parse().unwrap_or(0);
        if pct > 10000 {
            errors.push(ValidationError::error_pos(
                "E231",
                line,
                (476, 480),
                "PORCENTAJE_PARTICIPACION",
                &format!(
                    "PORCENTAJE_PARTICIPACION fuera de rango: {} (max 100.00%)",
                    pct
                ),
            ));
        }
    }

    // E232: BLANCOS pos 481-500 must be all spaces
    let blancos = field(raw, 481, 500);
    if !is_blank(&blancos) {
        errors.push(ValidationError::error_pos(
            "E232",
            line,
            (481, 500),
            "BLANCOS",
            "Posiciones 481-500 contienen caracteres no blancos",
        ));
    }

    // --- Warnings ---

    // W001: FECHA_INCORPORACION future to ejercicio
    if !is_empty_date(&fecha_inc) && is_numeric(&fecha_inc) && fecha_inc.len() == 8 {
        let fecha_year: u32 = fecha_inc[0..4].parse().unwrap_or(0);
        let ejercicio: u32 = record.ejercicio.parse().unwrap_or(0);
        if fecha_year > ejercicio {
            warnings.push(ValidationError::warning(
                "W001",
                line,
                "FECHA_INCORPORACION",
                &format!(
                    "FECHA_INCORPORACION ({}) futura al ejercicio ({})",
                    fecha_inc, record.ejercicio
                ),
            ));
        }
    }

    // W002: VALORACION1 > 10.000.000€
    if is_numeric(val1.trim()) && !val1.trim().is_empty() {
        let v: i64 = val1.trim().parse().unwrap_or(0);
        if v > 1_000_000_000 {
            // 10M€ = 1_000_000_000 centavos
            warnings.push(ValidationError::warning(
                "W002",
                line,
                "VALORACION1",
                &format!("VALORACION1 > 10.000.000€: {:.2}€", v as f64 / 100.0),
            ));
        }
    }

    // W003: PORCENTAJE_PARTICIPACION = 0
    if is_numeric(&pct_str) {
        let pct: u32 = pct_str.parse().unwrap_or(0);
        if pct == 0 {
            warnings.push(ValidationError::warning(
                "W003",
                line,
                "PORCENTAJE_PARTICIPACION",
                "PORCENTAJE_PARTICIPACION = 0",
            ));
        }
    }

    // W004: CODIGO_BIC format check (pos 145-155)
    let bic = field(raw, 145, 155);
    if clave_bien == 'C' && !is_blank(&bic) {
        let bic_trimmed = bic.trim();
        let bic_ok = (bic_trimmed.len() == 8 || bic_trimmed.len() == 11)
            && bic_trimmed.chars().all(|c| c.is_ascii_alphanumeric());
        if !bic_ok {
            warnings.push(ValidationError::warning(
                "W004",
                line,
                "CODIGO_BIC",
                &format!("CODIGO_BIC formato incorrecto: '{}'", bic_trimmed),
            ));
        }
    }
}
