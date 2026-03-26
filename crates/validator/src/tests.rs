use crate::*;

/// Build a valid Type 1 record (500 chars).
fn make_t1(
    nif: &str,
    nombre: &str,
    total_t2: usize,
    sum_val1: i64,
    sum_val2: i64,
) -> String {
    make_t1_full(nif, nombre, total_t2, sum_val1, sum_val2, ' ', ' ', "0000000000000")
}

fn make_t1_full(
    nif: &str,
    nombre: &str,
    total_t2: usize,
    sum_val1: i64,
    sum_val2: i64,
    dec_comp: char,
    dec_sust: char,
    num_dec_anterior: &str,
) -> String {
    let mut rec = String::with_capacity(500);
    rec.push('1');
    rec.push_str("720");
    rec.push_str("2024");
    rec.push_str(&format!("{:<9}", nif));
    rec.push_str(&format!("{:<40}", nombre));
    rec.push('T');
    rec.push_str("912345678");
    rec.push_str(&format!("{:<40}", "CONTACTO"));
    rec.push_str("7200000000001");
    rec.push(dec_comp);
    rec.push(dec_sust);
    rec.push_str(&format!("{:<13}", num_dec_anterior));
    rec.push_str(&format!("{:09}", total_t2));
    if sum_val1 < 0 { rec.push('N'); } else { rec.push(' '); }
    rec.push_str(&format!("{:017}", sum_val1.unsigned_abs()));
    if sum_val2 < 0 { rec.push('N'); } else { rec.push(' '); }
    rec.push_str(&format!("{:017}", sum_val2.unsigned_abs()));
    rec.push_str(&" ".repeat(320));
    assert_eq!(rec.chars().count(), 500, "T1 record must be 500 chars");
    rec
}

/// Build a valid Type 2 record (500 chars) for a C (cuenta) bien.
fn make_t2_cuenta(
    nif_declarante: &str,
    nif_declarado: &str,
    nombre: &str,
    val1: i64,
    val2: i64,
) -> String {
    make_t2_full(
        nif_declarante, nif_declarado, nombre, val1, val2,
        '1', 'C', '1', "CH", '0', "            ", 'I',
        "20240101", 'A', "00000000", ' ', "000000000000", ' ', "10000",
    )
}

#[allow(clippy::too_many_arguments)]
fn make_t2_full(
    nif_declarante: &str,
    nif_declarado: &str,
    nombre: &str,
    val1: i64,
    val2: i64,
    clave_cond: char,
    clave_bien: char,
    subclave: char,
    codigo_pais: &str,
    clave_id: char,
    id_valores: &str,
    clave_id_cuenta: char,
    fecha_inc: &str,
    origen: char,
    fecha_ext: &str,
    clave_repr: char,
    num_valores: &str,
    clave_inmueble: char,
    porcentaje: &str,
) -> String {
    let mut rec = String::with_capacity(500);
    // pos 1
    rec.push('2');
    // pos 2-4
    rec.push_str("720");
    // pos 5-8
    rec.push_str("2024");
    // pos 9-17
    rec.push_str(&format!("{:<9}", nif_declarante));
    // pos 18-26
    rec.push_str(&format!("{:<9}", nif_declarado));
    // pos 27-35: nif rep legal (blank)
    rec.push_str("         ");
    // pos 36-75: nombre (40 chars)
    rec.push_str(&format!("{:<40}", nombre));
    // pos 76: clave condicion
    rec.push(clave_cond);
    // pos 77-101: tipo titularidad (25 blanks)
    rec.push_str(&" ".repeat(25));
    // pos 102: clave tipo bien
    rec.push(clave_bien);
    // pos 103: subclave
    rec.push(subclave);
    // pos 104-128: tipo derecho real (25 blanks)
    rec.push_str(&" ".repeat(25));
    // pos 129-130: codigo pais
    rec.push_str(&format!("{:<2}", codigo_pais));
    // pos 131: clave identificacion
    rec.push(clave_id);
    // pos 132-143: identificacion valores (12 chars)
    rec.push_str(&format!("{:<12}", id_valores));
    // pos 144: clave id cuenta
    rec.push(clave_id_cuenta);
    // pos 145-155: codigo BIC (11 chars)
    rec.push_str("UBSWCHZH80X");
    // pos 156-189: codigo cuenta (34 chars)
    rec.push_str(&format!("{:<34}", "CH9300762011623852957"));
    // pos 190-230: identificacion entidad (41 chars)
    rec.push_str(&format!("{:<41}", "UBS SWITZERLAND AG"));
    // pos 231-250: NIF pais residencia (20 chars)
    rec.push_str(&format!("{:<20}", "CHE101995734"));
    // pos 251-302: nombre via (52 chars)
    rec.push_str(&format!("{:<52}", "BAHNHOFSTRASSE 45"));
    // pos 303-342: complemento (40 chars)
    rec.push_str(&" ".repeat(40));
    // pos 343-372: poblacion (30 chars)
    rec.push_str(&format!("{:<30}", "ZURICH"));
    // pos 373-402: provincia (30 chars)
    rec.push_str(&format!("{:<30}", "ZURICH"));
    // pos 403-412: cod postal (10 chars)
    rec.push_str(&format!("{:<10}", "8001"));
    // pos 413-414: codigo pais domicilio
    rec.push_str("CH");
    // pos 415-422: fecha incorporacion
    rec.push_str(fecha_inc);
    // pos 423: origen
    rec.push(origen);
    // pos 424-431: fecha extincion
    rec.push_str(fecha_ext);
    // pos 432: signo val1
    if val1 < 0 { rec.push('N'); } else { rec.push(' '); }
    // pos 433-446: valoracion1 (14 digits)
    rec.push_str(&format!("{:014}", val1.unsigned_abs()));
    // pos 447: signo val2
    if val2 < 0 { rec.push('N'); } else { rec.push(' '); }
    // pos 448-461: valoracion2 (14 digits)
    rec.push_str(&format!("{:014}", val2.unsigned_abs()));
    // pos 462: clave represent valores
    rec.push(clave_repr);
    // pos 463-474: num valores (12 chars)
    rec.push_str(&format!("{:<12}", num_valores));
    // pos 475: clave tipo inmueble
    rec.push(clave_inmueble);
    // pos 476-480: porcentaje participacion
    rec.push_str(&format!("{:>5}", porcentaje));
    // pos 481-500: blancos (20)
    rec.push_str(&" ".repeat(20));
    assert_eq!(rec.chars().count(), 500, "T2 record must be 500 chars, got {}", rec.chars().count());
    rec
}

fn build_file(records: &[String]) -> Vec<u8> {
    let text = records.join("\r\n") + "\r\n";
    let (bytes, _, _) = encoding_rs::WINDOWS_1252.encode(&text);
    bytes.to_vec()
}

fn has_error(result: &ValidationResult, code: &str) -> bool {
    result.errors.iter().any(|e| e.code == code)
}

fn has_warning(result: &ValidationResult, code: &str) -> bool {
    result.warnings.iter().any(|e| e.code == code)
}

// ─── Integration tests against fixture files ─────────────────────────

#[test]
fn test_fixture_valid() {
    let bytes = include_bytes!("../../../tests/fixtures/valid.720");
    let result = validate(bytes);
    assert!(result.is_valid, "valid.720 should pass, errors: {:?}", result.errors);
    let s = result.summary.unwrap();
    assert_eq!(s.total_registros_t2_real, 2);
    assert_eq!(s.ejercicio, "2024");
}

#[test]
fn test_fixture_invalid() {
    let bytes = include_bytes!("../../../tests/fixtures/invalid.720");
    let result = validate(bytes);
    assert!(!result.is_valid, "invalid.720 should fail");
    // Should have NIF error, T2 count mismatch, and bad date
    assert!(has_error(&result, "E102") || has_error(&result, "E107") || has_error(&result, "E218"),
        "Expected at least one known error, got: {:?}", result.errors);
}

// ─── Structural (Fatal) ──────────────────────────────────────────────

#[test]
fn test_valid_single_record() {
    let val1: i64 = 4144627;
    let val2: i64 = 0;
    let t1 = make_t1("12345678Z", "GARCIA LOPEZ JUAN", 1, val1, val2);
    let t2 = make_t2_cuenta("12345678Z", "12345678Z", "GARCIA LOPEZ JUAN", val1, val2);
    let bytes = build_file(&[t1, t2]);
    let result = validate(&bytes);
    assert!(result.is_valid, "Expected valid, errors: {:?}", result.errors);
    let summary = result.summary.unwrap();
    assert_eq!(summary.ejercicio, "2024");
    assert_eq!(summary.nif_declarante, "12345678Z");
    assert_eq!(summary.total_registros_t2_real, 1);
}

#[test]
fn test_empty_file() {
    let result = validate(b"");
    assert!(!result.is_valid);
    assert!(has_error(&result, "E003"));
}

#[test]
fn test_wrong_line_length() {
    let bytes = b"1720202412345678Z\r\n";
    let result = validate(bytes);
    assert!(!result.is_valid);
    assert!(has_error(&result, "E002"));
}

#[test]
fn test_first_record_not_type1() {
    let t2 = make_t2_cuenta("12345678Z", "12345678Z", "NOMBRE", 0, 0);
    let bytes = build_file(&[t2]);
    let result = validate(&bytes);
    assert!(!result.is_valid);
    assert!(has_error(&result, "E003"));
}

#[test]
fn test_multiple_type1_records() {
    let t1a = make_t1("12345678Z", "NOMBRE A", 0, 0, 0);
    let t1b = make_t1("12345678Z", "NOMBRE B", 0, 0, 0);
    let bytes = build_file(&[t1a, t1b]);
    let result = validate(&bytes);
    assert!(!result.is_valid);
    assert!(has_error(&result, "E004"));
}

#[test]
fn test_unknown_record_type() {
    // Valid T1 followed by a record with type '3'
    let t1 = make_t1("12345678Z", "NOMBRE", 0, 0, 0);
    let mut bad = make_t2_cuenta("12345678Z", "12345678Z", "NOMBRE", 0, 0);
    unsafe { bad.as_bytes_mut()[0] = b'3'; }
    let bytes = build_file(&[t1, bad]);
    let result = validate(&bytes);
    assert!(!result.is_valid);
    assert!(has_error(&result, "E005"));
}

#[test]
fn test_modelo_not_720() {
    let mut t1 = make_t1("12345678Z", "GARCIA LOPEZ JUAN", 0, 0, 0);
    unsafe {
        let bytes = t1.as_bytes_mut();
        bytes[1] = b'3';
        bytes[2] = b'0';
        bytes[3] = b'3';
    }
    let file_bytes = build_file(&[t1]);
    let result = validate(&file_bytes);
    assert!(!result.is_valid);
    assert!(has_error(&result, "E006"));
}

#[test]
fn test_ejercicio_non_numeric() {
    let mut t1 = make_t1("12345678Z", "NOMBRE", 0, 0, 0);
    unsafe {
        let bytes = t1.as_bytes_mut();
        bytes[4] = b'A';
    }
    let bytes = build_file(&[t1]);
    let result = validate(&bytes);
    assert!(!result.is_valid);
    assert!(has_error(&result, "E007"));
}

// ─── Tipo 1 (E101-E110) ─────────────────────────────────────────────

#[test]
fn test_e101_tipo_soporte() {
    let mut t1 = make_t1("12345678Z", "NOMBRE", 0, 0, 0);
    // pos 58 (index 57) must be 'T'
    unsafe { t1.as_bytes_mut()[57] = b'X'; }
    let bytes = build_file(&[t1]);
    let result = validate(&bytes);
    assert!(has_error(&result, "E101"));
}

#[test]
fn test_invalid_nif_declarante() {
    let t1 = make_t1("12345678A", "GARCIA LOPEZ JUAN", 0, 0, 0);
    let bytes = build_file(&[t1]);
    let result = validate(&bytes);
    assert!(!result.is_valid);
    assert!(has_error(&result, "E102"));
}

#[test]
fn test_e103_empty_nombre() {
    let t1 = make_t1("12345678Z", "", 0, 0, 0);
    let bytes = build_file(&[t1]);
    let result = validate(&bytes);
    assert!(has_error(&result, "E103"));
}

#[test]
fn test_e104_num_identificativo() {
    let mut t1 = make_t1("12345678Z", "NOMBRE", 0, 0, 0);
    // pos 108-110 (index 107-109), change from "720" to "999"
    unsafe {
        let bytes = t1.as_bytes_mut();
        bytes[107] = b'9';
        bytes[108] = b'9';
        bytes[109] = b'9';
    }
    let bytes = build_file(&[t1]);
    let result = validate(&bytes);
    assert!(has_error(&result, "E104"));
}

#[test]
fn test_e105_both_complementaria_and_sustitutiva() {
    let t1 = make_t1_full(
        "12345678Z", "NOMBRE", 0, 0, 0,
        'C', 'S', "7200000000002",
    );
    let bytes = build_file(&[t1]);
    let result = validate(&bytes);
    assert!(has_error(&result, "E105"));
}

#[test]
fn test_e106_sustitutiva_without_anterior() {
    let t1 = make_t1_full(
        "12345678Z", "NOMBRE", 0, 0, 0,
        ' ', 'S', "0000000000000",
    );
    let bytes = build_file(&[t1]);
    let result = validate(&bytes);
    assert!(has_error(&result, "E106"));
}

#[test]
fn test_e110_blancos_not_blank() {
    let mut t1 = make_t1("12345678Z", "NOMBRE", 0, 0, 0);
    // Put non-blank at pos 181 (index 180)
    unsafe { t1.as_bytes_mut()[180] = b'X'; }
    let bytes = build_file(&[t1]);
    let result = validate(&bytes);
    assert!(has_error(&result, "E110"));
}

// ─── Cross-validations (E107-E109) ──────────────────────────────────

#[test]
fn test_total_t2_mismatch() {
    let val1: i64 = 4144627;
    let t1 = make_t1("12345678Z", "GARCIA LOPEZ JUAN", 2, val1, 0);
    let t2 = make_t2_cuenta("12345678Z", "12345678Z", "GARCIA LOPEZ JUAN", val1, 0);
    let bytes = build_file(&[t1, t2]);
    let result = validate(&bytes);
    assert!(!result.is_valid);
    assert!(has_error(&result, "E107"));
}

#[test]
fn test_val1_sum_mismatch() {
    let t1 = make_t1("12345678Z", "GARCIA LOPEZ JUAN", 1, 100, 0);
    let t2 = make_t2_cuenta("12345678Z", "12345678Z", "GARCIA LOPEZ JUAN", 200, 0);
    let bytes = build_file(&[t1, t2]);
    let result = validate(&bytes);
    assert!(!result.is_valid);
    assert!(has_error(&result, "E108"));
}

#[test]
fn test_val2_sum_mismatch() {
    let t1 = make_t1("12345678Z", "NOMBRE", 1, 100, 50);
    let t2 = make_t2_cuenta("12345678Z", "12345678Z", "NOMBRE", 100, 999);
    let bytes = build_file(&[t1, t2]);
    let result = validate(&bytes);
    assert!(has_error(&result, "E109"));
}

#[test]
fn test_multiple_t2_records() {
    let val1_a: i64 = 1000000;
    let val1_b: i64 = 2000000;
    let total_val1 = val1_a + val1_b;
    let t1 = make_t1("12345678Z", "GARCIA LOPEZ JUAN", 2, total_val1, 0);
    let t2a = make_t2_cuenta("12345678Z", "12345678Z", "GARCIA LOPEZ JUAN", val1_a, 0);
    let t2b = make_t2_cuenta("12345678Z", "00000000T", "PEREZ MARTINEZ ANA", val1_b, 0);
    let bytes = build_file(&[t1, t2a, t2b]);
    let result = validate(&bytes);
    assert!(result.is_valid, "Expected valid, errors: {:?}", result.errors);
    assert_eq!(result.summary.unwrap().total_registros_t2_real, 2);
}

// ─── Tipo 2 (E201-E233) ─────────────────────────────────────────────

#[test]
fn test_e201_nif_declarante_mismatch() {
    let t1 = make_t1("12345678Z", "NOMBRE", 1, 100, 0);
    let t2 = make_t2_cuenta("00000000T", "12345678Z", "NOMBRE", 100, 0);
    let bytes = build_file(&[t1, t2]);
    let result = validate(&bytes);
    assert!(has_error(&result, "E201"));
}

#[test]
fn test_e202_invalid_nif_declarado() {
    let t1 = make_t1("12345678Z", "NOMBRE", 1, 100, 0);
    let t2 = make_t2_cuenta("12345678Z", "XXXXXXXXX", "NOMBRE", 100, 0);
    let bytes = build_file(&[t1, t2]);
    let result = validate(&bytes);
    assert!(has_error(&result, "E202"));
}

#[test]
fn test_e203_empty_nombre_declarado() {
    let t1 = make_t1("12345678Z", "NOMBRE", 1, 100, 0);
    let t2 = make_t2_cuenta("12345678Z", "12345678Z", "", 100, 0);
    let bytes = build_file(&[t1, t2]);
    let result = validate(&bytes);
    assert!(has_error(&result, "E203"));
}

#[test]
fn test_e204_clave_condicion_out_of_range() {
    let t1 = make_t1("12345678Z", "NOMBRE", 1, 100, 0);
    let t2 = make_t2_full(
        "12345678Z", "12345678Z", "NOMBRE", 100, 0,
        '0', 'C', '1', "CH", '0', "            ", 'I',
        "20240101", 'A', "00000000", ' ', "000000000000", ' ', "10000",
    );
    let bytes = build_file(&[t1, t2]);
    let result = validate(&bytes);
    assert!(has_error(&result, "E204"));
}

#[test]
fn test_e206_invalid_clave_tipo_bien() {
    let t1 = make_t1("12345678Z", "NOMBRE", 1, 100, 0);
    let t2 = make_t2_full(
        "12345678Z", "12345678Z", "NOMBRE", 100, 0,
        '1', 'X', '1', "CH", '0', "            ", ' ',
        "20240101", 'A', "00000000", ' ', "000000000000", ' ', "10000",
    );
    let bytes = build_file(&[t1, t2]);
    let result = validate(&bytes);
    assert!(has_error(&result, "E206"));
}

#[test]
fn test_e207_invalid_subclave() {
    let t1 = make_t1("12345678Z", "NOMBRE", 1, 100, 0);
    // C with subclave 9 (invalid, must be 1-5)
    let t2 = make_t2_full(
        "12345678Z", "12345678Z", "NOMBRE", 100, 0,
        '1', 'C', '9', "CH", '0', "            ", 'I',
        "20240101", 'A', "00000000", ' ', "000000000000", ' ', "10000",
    );
    let bytes = build_file(&[t1, t2]);
    let result = validate(&bytes);
    assert!(has_error(&result, "E207"));
}

#[test]
fn test_e208_subclave_not_zero_for_i() {
    let t1 = make_t1("12345678Z", "NOMBRE", 1, 100, 0);
    let t2 = make_t2_full(
        "12345678Z", "12345678Z", "NOMBRE", 100, 0,
        '1', 'I', '1', "CH", '0', "            ", ' ',
        "20240101", 'A', "00000000", 'A', "000100000000", ' ', "10000",
    );
    let bytes = build_file(&[t1, t2]);
    let result = validate(&bytes);
    assert!(has_error(&result, "E208"));
}

#[test]
fn test_e220_origen_invalid() {
    let t1 = make_t1("12345678Z", "NOMBRE", 1, 100, 0);
    let t2 = make_t2_full(
        "12345678Z", "12345678Z", "NOMBRE", 100, 0,
        '1', 'C', '1', "CH", '0', "            ", 'I',
        "20240101", 'X', "00000000", ' ', "000000000000", ' ', "10000",
    );
    let bytes = build_file(&[t1, t2]);
    let result = validate(&bytes);
    assert!(has_error(&result, "E220"));
}

#[test]
fn test_e221_fecha_extincion_zeros_when_origen_c() {
    let t1 = make_t1("12345678Z", "NOMBRE", 1, 100, 0);
    let t2 = make_t2_full(
        "12345678Z", "12345678Z", "NOMBRE", 100, 0,
        '1', 'C', '1', "CH", '0', "            ", 'I',
        "20240101", 'C', "00000000", ' ', "000000000000", ' ', "10000",
    );
    let bytes = build_file(&[t1, t2]);
    let result = validate(&bytes);
    assert!(has_error(&result, "E221"));
}

#[test]
fn test_e222_fecha_extincion_not_zeros_when_origen_a() {
    let t1 = make_t1("12345678Z", "NOMBRE", 1, 100, 0);
    let t2 = make_t2_full(
        "12345678Z", "12345678Z", "NOMBRE", 100, 0,
        '1', 'C', '1', "CH", '0', "            ", 'I',
        "20240101", 'A', "20241231", ' ', "000000000000", ' ', "10000",
    );
    let bytes = build_file(&[t1, t2]);
    let result = validate(&bytes);
    assert!(has_error(&result, "E222"));
}

#[test]
fn test_e233_ejercicio_mismatch() {
    let t1 = make_t1("12345678Z", "NOMBRE", 1, 100, 0);
    let mut t2 = make_t2_cuenta("12345678Z", "12345678Z", "NOMBRE", 100, 0);
    // Change ejercicio to 2025 at pos 5-8 (index 4-7)
    unsafe {
        let b = t2.as_bytes_mut();
        b[4] = b'2'; b[5] = b'0'; b[6] = b'2'; b[7] = b'5';
    }
    let bytes = build_file(&[t1, t2]);
    let result = validate(&bytes);
    assert!(has_error(&result, "E233"));
}

#[test]
fn test_e232_blancos_not_blank() {
    let mut t2 = make_t2_cuenta("12345678Z", "12345678Z", "NOMBRE", 100, 0);
    // pos 481 (index 480)
    unsafe { t2.as_bytes_mut()[480] = b'X'; }
    let t1 = make_t1("12345678Z", "NOMBRE", 1, 100, 0);
    let bytes = build_file(&[t1, t2]);
    let result = validate(&bytes);
    assert!(has_error(&result, "E232"));
}

// ─── Warnings (W001-W005) ───────────────────────────────────────────

#[test]
fn test_w001_future_fecha_incorporacion() {
    let t1 = make_t1("12345678Z", "NOMBRE", 1, 100, 0);
    let t2 = make_t2_full(
        "12345678Z", "12345678Z", "NOMBRE", 100, 0,
        '1', 'C', '1', "CH", '0', "            ", 'I',
        "20250601", 'A', "00000000", ' ', "000000000000", ' ', "10000",
    );
    let bytes = build_file(&[t1, t2]);
    let result = validate(&bytes);
    assert!(has_warning(&result, "W001"));
}

#[test]
fn test_w003_zero_porcentaje() {
    let t1 = make_t1("12345678Z", "NOMBRE", 1, 100, 0);
    let t2 = make_t2_full(
        "12345678Z", "12345678Z", "NOMBRE", 100, 0,
        '1', 'C', '1', "CH", '0', "            ", 'I',
        "20240101", 'A', "00000000", ' ', "000000000000", ' ', "00000",
    );
    let bytes = build_file(&[t1, t2]);
    let result = validate(&bytes);
    assert!(has_warning(&result, "W003"));
}

// ─── Edge cases ─────────────────────────────────────────────────────

#[test]
fn test_zero_t2_records() {
    let t1 = make_t1("12345678Z", "NOMBRE", 0, 0, 0);
    let bytes = build_file(&[t1]);
    let result = validate(&bytes);
    assert!(result.is_valid, "T1-only file with 0 T2 should be valid, errors: {:?}", result.errors);
    let s = result.summary.unwrap();
    assert_eq!(s.total_registros_t2_real, 0);
    assert_eq!(s.total_registros_t2_declarado, 0);
}

#[test]
fn test_negative_values() {
    let t1 = make_t1("12345678Z", "NOMBRE", 1, -500, -300);
    let t2 = make_t2_cuenta("12345678Z", "12345678Z", "NOMBRE", -500, -300);
    let bytes = build_file(&[t1, t2]);
    let result = validate(&bytes);
    assert!(result.is_valid, "Negative values should be valid, errors: {:?}", result.errors);
    let s = result.summary.unwrap();
    assert!((s.suma_val1 - (-5.0)).abs() < 0.01);
    assert!((s.suma_val2 - (-3.0)).abs() < 0.01);
}

#[test]
fn test_lf_line_endings() {
    // File with LF instead of CRLF should still work
    let t1 = make_t1("12345678Z", "NOMBRE", 0, 0, 0);
    let text = format!("{}\n", t1);
    let (bytes, _, _) = encoding_rs::WINDOWS_1252.encode(&text);
    let result = validate(&bytes.to_vec());
    assert!(result.is_valid, "LF-only line endings should work, errors: {:?}", result.errors);
}
