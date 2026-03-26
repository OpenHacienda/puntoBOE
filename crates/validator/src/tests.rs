use crate::*;

/// Build a valid Type 1 record (500 chars).
fn make_t1(
    nif: &str,
    nombre: &str,
    total_t2: usize,
    sum_val1: i64,
    sum_val2: i64,
) -> String {
    let mut rec = String::with_capacity(500);
    // pos 1: tipo
    rec.push('1');
    // pos 2-4: modelo
    rec.push_str("720");
    // pos 5-8: ejercicio
    rec.push_str("2024");
    // pos 9-17: NIF (9 chars, right-padded with spaces)
    rec.push_str(&format!("{:<9}", nif));
    // pos 18-57: nombre (40 chars)
    rec.push_str(&format!("{:<40}", nombre));
    // pos 58: tipo soporte
    rec.push('T');
    // pos 59-67: telefono (9 digits)
    rec.push_str("912345678");
    // pos 68-107: persona contacto (40 chars)
    rec.push_str(&format!("{:<40}", "CONTACTO"));
    // pos 108-120: num identificativo (13 chars, starts with 720)
    rec.push_str("7200000000001");
    // pos 121: dec complementaria
    rec.push(' ');
    // pos 122: dec sustitutiva
    rec.push(' ');
    // pos 123-135: num dec anterior (13 zeros)
    rec.push_str("0000000000000");
    // pos 136-144: total registros t2 (9 digits)
    rec.push_str(&format!("{:09}", total_t2));
    // pos 145: signo val1
    if sum_val1 < 0 { rec.push('N'); } else { rec.push(' '); }
    // pos 146-162: suma val1 (17 digits)
    rec.push_str(&format!("{:017}", sum_val1.unsigned_abs()));
    // pos 163: signo val2
    if sum_val2 < 0 { rec.push('N'); } else { rec.push(' '); }
    // pos 164-180: suma val2 (17 digits)
    rec.push_str(&format!("{:017}", sum_val2.unsigned_abs()));
    // pos 181-500: blanks (320 chars)
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
    rec.push('1');
    // pos 77-101: tipo titularidad (25 blanks)
    rec.push_str(&" ".repeat(25));
    // pos 102: clave tipo bien
    rec.push('C');
    // pos 103: subclave
    rec.push('1');
    // pos 104-128: tipo derecho real (25 blanks)
    rec.push_str(&" ".repeat(25));
    // pos 129-130: codigo pais
    rec.push_str("CH");
    // pos 131: clave identificacion (0 for C)
    rec.push('0');
    // pos 132-143: identificacion valores (12 blanks for C)
    rec.push_str("            ");
    // pos 144: clave id cuenta
    rec.push('I');
    // pos 145-155: codigo BIC (11 chars)
    rec.push_str("UBSWCHZH80X");
    // pos 156-189: codigo cuenta (34 chars, IBAN)
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
    // pos 415-422: fecha incorporacion (AAAAMMDD)
    rec.push_str("20240101");
    // pos 423: origen
    rec.push('A');
    // pos 424-431: fecha extincion (zeros for A)
    rec.push_str("00000000");
    // pos 432: signo val1
    if val1 < 0 { rec.push('N'); } else { rec.push(' '); }
    // pos 433-446: valoracion1 (14 digits)
    rec.push_str(&format!("{:014}", val1.unsigned_abs()));
    // pos 447: signo val2
    if val2 < 0 { rec.push('N'); } else { rec.push(' '); }
    // pos 448-461: valoracion2 (14 digits)
    rec.push_str(&format!("{:014}", val2.unsigned_abs()));
    // pos 462: clave represent valores (blank for C)
    rec.push(' ');
    // pos 463-474: num valores (12 zeros for C)
    rec.push_str("000000000000");
    // pos 475: clave tipo inmueble (blank for C)
    rec.push(' ');
    // pos 476-480: porcentaje participacion
    rec.push_str("10000");
    // pos 481-500: blancos (20)
    rec.push_str(&" ".repeat(20));
    assert_eq!(rec.chars().count(), 500, "T2 record must be 500 chars");
    rec
}

fn build_file(records: &[String]) -> Vec<u8> {
    let text = records.join("\r\n") + "\r\n";
    // Encode to Windows-1252
    let (bytes, _, _) = encoding_rs::WINDOWS_1252.encode(&text);
    bytes.to_vec()
}

#[test]
fn test_valid_single_record() {
    let val1: i64 = 4144627; // 41446.27€
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
    assert!(result.errors.iter().any(|e| e.code == "E003"));
}

#[test]
fn test_wrong_line_length() {
    let bytes = b"1720202412345678Z\r\n";
    let result = validate(bytes);
    assert!(!result.is_valid);
    assert!(result.errors.iter().any(|e| e.code == "E002"));
}

#[test]
fn test_invalid_nif_declarante() {
    let t1 = make_t1("12345678A", "GARCIA LOPEZ JUAN", 0, 0, 0);
    let bytes = build_file(&[t1]);
    let result = validate(&bytes);
    assert!(!result.is_valid);
    assert!(result.errors.iter().any(|e| e.code == "E102"));
}

#[test]
fn test_modelo_not_720() {
    let mut t1 = make_t1("12345678Z", "GARCIA LOPEZ JUAN", 0, 0, 0);
    // Replace modelo at pos 2-4
    unsafe {
        let bytes = t1.as_bytes_mut();
        bytes[1] = b'3';
        bytes[2] = b'0';
        bytes[3] = b'3';
    }
    let file_bytes = build_file(&[t1]);
    let result = validate(&file_bytes);
    assert!(!result.is_valid);
    assert!(result.errors.iter().any(|e| e.code == "E006"));
}

#[test]
fn test_total_t2_mismatch() {
    let val1: i64 = 4144627;
    // T1 says 2 T2 records but only 1 exists
    let t1 = make_t1("12345678Z", "GARCIA LOPEZ JUAN", 2, val1, 0);
    let t2 = make_t2_cuenta("12345678Z", "12345678Z", "GARCIA LOPEZ JUAN", val1, 0);
    let bytes = build_file(&[t1, t2]);
    let result = validate(&bytes);
    assert!(!result.is_valid);
    assert!(result.errors.iter().any(|e| e.code == "E107"));
}

#[test]
fn test_val1_sum_mismatch() {
    // T1 says sum=100 but T2 has val1=200
    let t1 = make_t1("12345678Z", "GARCIA LOPEZ JUAN", 1, 100, 0);
    let t2 = make_t2_cuenta("12345678Z", "12345678Z", "GARCIA LOPEZ JUAN", 200, 0);
    let bytes = build_file(&[t1, t2]);
    let result = validate(&bytes);
    assert!(!result.is_valid);
    assert!(result.errors.iter().any(|e| e.code == "E108"));
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
