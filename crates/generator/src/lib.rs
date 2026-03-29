// ── Record layout (1-based, inclusive) ───────────────────────────────────────
//
// TIPO 1 (500 chars)
//  1       TIPO_REGISTRO = '1'
//  2– 4    MODELO        = "720"
//  5– 8    EJERCICIO
//  9–17    NIF_DECLARANTE
// 18–57    APELLIDOS_NOMBRE              (40)
// 58       TIPO_SOPORTE  = 'T'
// 59–67    NIF_PRESENTADOR               (9)
// 68–107   NOM_PRESENTADOR               (40)
// 108–120  NUM_IDENTIFICATIVO            (13, starts "720")
// 121      DEC_COMPLEMENTARIA            ' '|'C'
// 122      DEC_SUSTITUTIVA               ' '|'S'
// 123–135  NUM_DEC_ANTERIOR              (13 zeros if not sustitutiva)
// 136–144  TOTAL_REGISTROS_T2            (9 digits)
// 145      SIGNO_VAL1                    ' '|'N'
// 146–162  SUMA_VAL1                     (17 digits, cents)
// 163      SIGNO_VAL2                    ' '|'N'
// 164–180  SUMA_VAL2                     (17 digits, cents)
// 181–500  BLANCOS
//
// TIPO 2 (500 chars)
//  1       TIPO_REGISTRO = '2'
//  2– 4    MODELO
//  5– 8    EJERCICIO
//  9–17    NIF_DECLARANTE
// 18–26    NIF_DECLARADO                 (9)
// 27–35    NIF_REPRESENTANTE             (9, blanks)
// 36–75    APELLIDOS_NOMBRE_DECLARADO    (40)
// 76       CLAVE_CONDICION               '1'–'8'
// 77–101   TIPO_TITULARIDAD              (25, blanks)
// 102      CLAVE_TIPO_BIEN               C|V|I|S|B
// 103      SUBCLAVE
// 104–128  BLANCOS                       (25)
// 129–130  CODIGO_PAIS                   (2, ISO-3166)
// 131      CLAVE_IDENTIFICACION          '0'|'1'|'2'
// 132–143  IDENTIFICACION_VALORES        (12)
// 144      CLAVE_ID_CUENTA               'I'|'O' for C, ' ' otherwise
// 145–155  CODIGO_BIC                    (11)
// 156–189  IDENTIFICACION_CUENTA         (34, IBAN)
// 190–230  IDENTIFICACION_ENTIDAD        (41)
// 231–250  NIF_FISCAL_PAIS               (20)
// 251–290  DOMICILIO                     (40)
// 291–342  BLANCOS                       (52)
// 343–372  LOCALIDAD                     (30)
// 373–402  MUNICIPIO                     (30)
// 403–412  CODIGO_POSTAL                 (10)
// 413–414  PAIS_DOMICILIO                (2)
// 415–422  FECHA_INCORPORACION           (8, AAAAMMDD)
// 423      ORIGEN                        A|M|C
// 424–431  FECHA_EXTINCION               (8, zeros when ORIGEN≠C)
// 432      SIGNO_VAL1                    ' '|'N'
// 433–446  VALORACION1                   (14 digits, cents)
// 447      SIGNO_VAL2                    ' '|'N'
// 448–461  VALORACION2                   (14 digits, cents)
// 462      CLAVE_REPRESENT_VALORES       A|B for V/I, ' ' otherwise
// 463–474  NUM_VALORES                   (12 digits)
// 475      CLAVE_TIPO_INMUEBLE           U|R for B, ' ' otherwise
// 476–480  PORCENTAJE_PARTICIPACION      (5 digits, 0–10000)
// 481–500  BLANCOS                       (20)

use encoding_rs::WINDOWS_1252;

// ── Data structs ──────────────────────────────────────────────────────────────

/// All editable fields for the Tipo 1 (header) record.
#[derive(Debug, Clone, PartialEq)]
pub struct Tipo1Fields {
    /// 4-digit year, e.g. "2024"
    pub ejercicio: String,
    /// 9-char NIF/NIE/CIF of the declarant
    pub nif_declarante: String,
    /// Up to 40 chars — apellidos y nombre or razón social
    pub nombre_declarante: String,
    /// 9-char NIF of the presenter. If empty, falls back to `nif_declarante`.
    pub nif_presentador: String,
    /// Up to 40 chars. If empty, falls back to `nombre_declarante`.
    pub nombre_presentador: String,
    /// 13 chars, must start with "720". Defaults to "7200000000001".
    pub num_identificativo: String,
    pub dec_complementaria: bool,
    pub dec_sustitutiva: bool,
    /// 13 digits. Only meaningful when `dec_sustitutiva` is true.
    pub num_dec_anterior: String,
}

impl Default for Tipo1Fields {
    fn default() -> Self {
        Self {
            ejercicio: "2024".to_string(),
            nif_declarante: String::new(),
            nombre_declarante: String::new(),
            nif_presentador: String::new(),
            nombre_presentador: String::new(),
            num_identificativo: "7200000000001".to_string(),
            dec_complementaria: false,
            dec_sustitutiva: false,
            num_dec_anterior: "0000000000000".to_string(),
        }
    }
}

/// All editable fields for a single Tipo 2 (asset) record.
#[derive(Debug, Clone, PartialEq)]
pub struct Tipo2Fields {
    /// 9-char NIF/NIE/CIF of the declared entity
    pub nif_declarado: String,
    /// 9-char NIF of the representative (blanks if none)
    pub nif_representante: String,
    /// Up to 40 chars
    pub nombre_declarado: String,
    /// '1'–'8'
    pub clave_condicion: char,
    /// C=Cuenta / V=Valores / I=IIC / S=Seguro / B=Inmueble
    pub clave_tipo_bien: char,
    pub subclave: char,
    /// 2-char ISO-3166 country code
    pub codigo_pais: String,
    /// '0'=none / '1'=ISIN / '2'=other
    pub clave_identificacion: char,
    /// Up to 12 chars (ISIN or other identifier)
    pub identificacion_valores: String,
    /// 'I'=IBAN / 'O'=other for C; ' ' otherwise
    pub clave_id_cuenta: char,
    /// Up to 11 chars BIC/SWIFT code
    pub codigo_bic: String,
    /// Up to 34 chars (IBAN)
    pub identificacion_cuenta: String,
    /// Up to 41 chars — entity name / bank name
    pub identificacion_entidad: String,
    /// Up to 20 chars — tax ID in country of residence
    pub nif_fiscal_pais: String,
    /// Up to 40 chars — street address
    pub domicilio: String,
    /// Up to 30 chars — city / locality
    pub localidad: String,
    /// Up to 30 chars — municipality / province
    pub municipio: String,
    /// Up to 10 chars — postal code
    pub codigo_postal: String,
    /// 2-char ISO country of the domicile
    pub pais_domicilio: String,
    /// 8 digits AAAAMMDD; "00000000" if unknown
    pub fecha_incorporacion: String,
    /// A=Alta / M=Modificación / C=Cancelación
    pub origen: char,
    /// 8 digits AAAAMMDD; "00000000" when origen ≠ C
    pub fecha_extincion: String,
    /// Euros (decimal). Stored as f64; encoded as cents in the file.
    pub valoracion1: f64,
    pub valoracion2: f64,
    /// A|B for V/I; ' ' for other asset types
    pub clave_represent_valores: char,
    /// Number of units / shares
    pub num_valores: u64,
    /// U=Urbano / R=Rústico for B; ' ' otherwise
    pub clave_tipo_inmueble: char,
    /// 0–10000 representing 0.00 %–100.00 %
    pub porcentaje_participacion: u32,
}

impl Default for Tipo2Fields {
    fn default() -> Self {
        Self {
            nif_declarado: String::new(),
            nif_representante: String::new(),
            nombre_declarado: String::new(),
            clave_condicion: '1',
            clave_tipo_bien: 'C',
            subclave: '1',
            codigo_pais: String::new(),
            clave_identificacion: '0',
            identificacion_valores: String::new(),
            clave_id_cuenta: 'I',
            codigo_bic: String::new(),
            identificacion_cuenta: String::new(),
            identificacion_entidad: String::new(),
            nif_fiscal_pais: String::new(),
            domicilio: String::new(),
            localidad: String::new(),
            municipio: String::new(),
            codigo_postal: String::new(),
            pais_domicilio: String::new(),
            fecha_incorporacion: "00000000".to_string(),
            origen: 'A',
            fecha_extincion: "00000000".to_string(),
            valoracion1: 0.0,
            valoracion2: 0.0,
            clave_represent_valores: ' ',
            num_valores: 0,
            clave_tipo_inmueble: ' ',
            porcentaje_participacion: 0,
        }
    }
}

// ── Generator ─────────────────────────────────────────────────────────────────

/// Build a valid Modelo 720 BOE file as ISO-8859-1 bytes.
///
/// The function:
/// 1. Encodes each Tipo 2 record into a 500-char line.
/// 2. Computes totals (count, Σval1, Σval2) and injects them into the Tipo 1 record.
/// 3. Joins all lines with CRLF, appends a trailing CRLF, and encodes as
///    Windows-1252 (superset of ISO-8859-1 accepted by the AEAT validator).
pub fn generate_file(t1: &Tipo1Fields, records: &[Tipo2Fields]) -> Vec<u8> {
    let ejercicio = pad_right(&t1.ejercicio, 4);
    let nif_decl = pad_right(&t1.nif_declarante, 9);

    // Build T2 lines and accumulate totals
    let mut sum_val1_cents: i64 = 0;
    let mut sum_val2_cents: i64 = 0;
    let mut t2_lines: Vec<String> = Vec::with_capacity(records.len());

    for rec in records {
        let v1 = (rec.valoracion1 * 100.0).round() as i64;
        let v2 = (rec.valoracion2 * 100.0).round() as i64;
        sum_val1_cents += v1;
        sum_val2_cents += v2;
        t2_lines.push(build_t2(&ejercicio, &nif_decl, rec));
    }

    let t1_line = build_t1(t1, &ejercicio, &nif_decl, records.len(), sum_val1_cents, sum_val2_cents);

    let mut lines = vec![t1_line];
    lines.extend(t2_lines);
    let text = lines.join("\r\n") + "\r\n";

    let (encoded, _, _) = WINDOWS_1252.encode(&text);
    encoded.into_owned()
}

fn build_t1(
    t1: &Tipo1Fields,
    ejercicio: &str,
    nif_decl: &str,
    total_t2: usize,
    sum_val1_cents: i64,
    sum_val2_cents: i64,
) -> String {
    let mut buf = vec![' '; 500];

    buf[0] = '1';
    wstr(&mut buf, 2, "720", 3);
    wstr(&mut buf, 5, ejercicio, 4);
    wstr(&mut buf, 9, nif_decl, 9);
    wstr(&mut buf, 18, &t1.nombre_declarante, 40);
    buf[57] = 'T';

    let nif_pres = if t1.nif_presentador.trim().is_empty() {
        nif_decl.to_string()
    } else {
        pad_right(&t1.nif_presentador, 9)
    };
    wstr(&mut buf, 59, &nif_pres, 9);

    let nom_pres = if t1.nombre_presentador.trim().is_empty() {
        t1.nombre_declarante.clone()
    } else {
        t1.nombre_presentador.clone()
    };
    wstr(&mut buf, 68, &nom_pres, 40);

    wstr(&mut buf, 108, &pad_right(&t1.num_identificativo, 13), 13);
    buf[120] = if t1.dec_complementaria { 'C' } else { ' ' };
    buf[121] = if t1.dec_sustitutiva { 'S' } else { ' ' };
    wstr(&mut buf, 123, &pad_left_zeros(&t1.num_dec_anterior, 13), 13);

    wstr(&mut buf, 136, &format!("{:09}", total_t2), 9);

    let (s1, v1) = encode_amount(sum_val1_cents, 17);
    buf[144] = s1;
    wstr(&mut buf, 146, &v1, 17);

    let (s2, v2) = encode_amount(sum_val2_cents, 17);
    buf[162] = s2;
    wstr(&mut buf, 164, &v2, 17);

    // pos 181–500 remain spaces
    buf.iter().collect()
}

fn build_t2(ejercicio: &str, nif_decl: &str, r: &Tipo2Fields) -> String {
    let mut buf = vec![' '; 500];

    buf[0] = '2';
    wstr(&mut buf, 2, "720", 3);
    wstr(&mut buf, 5, ejercicio, 4);
    wstr(&mut buf, 9, nif_decl, 9);
    wstr(&mut buf, 18, &pad_right(&r.nif_declarado, 9), 9);
    wstr(&mut buf, 27, &pad_right(&r.nif_representante, 9), 9);
    wstr(&mut buf, 36, &r.nombre_declarado, 40);
    buf[75] = r.clave_condicion;
    // pos 77–101 TIPO_TITULARIDAD — blanks (already spaces)
    buf[101] = r.clave_tipo_bien;
    buf[102] = r.subclave;
    // pos 104–128 blanks
    wstr(&mut buf, 129, &pad_right(&r.codigo_pais, 2), 2);
    buf[130] = r.clave_identificacion;
    wstr(&mut buf, 132, &pad_right(&r.identificacion_valores, 12), 12);
    buf[143] = r.clave_id_cuenta;
    wstr(&mut buf, 145, &pad_right(&r.codigo_bic, 11), 11);
    wstr(&mut buf, 156, &pad_right(&r.identificacion_cuenta, 34), 34);
    wstr(&mut buf, 190, &r.identificacion_entidad, 41);
    wstr(&mut buf, 231, &pad_right(&r.nif_fiscal_pais, 20), 20);
    wstr(&mut buf, 251, &r.domicilio, 40);
    // pos 291–342 blanks
    wstr(&mut buf, 343, &r.localidad, 30);
    wstr(&mut buf, 373, &r.municipio, 30);
    wstr(&mut buf, 403, &pad_right(&r.codigo_postal, 10), 10);
    wstr(&mut buf, 413, &pad_right(&r.pais_domicilio, 2), 2);
    wstr(&mut buf, 415, &norm_date(&r.fecha_incorporacion), 8);
    buf[422] = r.origen;
    wstr(&mut buf, 424, &norm_date(&r.fecha_extincion), 8);

    let v1 = (r.valoracion1 * 100.0).round() as i64;
    let (s1, d1) = encode_amount(v1, 14);
    buf[431] = s1;
    wstr(&mut buf, 433, &d1, 14);

    let v2 = (r.valoracion2 * 100.0).round() as i64;
    let (s2, d2) = encode_amount(v2, 14);
    buf[446] = s2;
    wstr(&mut buf, 448, &d2, 14);

    buf[461] = r.clave_represent_valores;
    wstr(&mut buf, 463, &format!("{:012}", r.num_valores), 12);
    buf[474] = r.clave_tipo_inmueble;
    wstr(&mut buf, 476, &format!("{:05}", r.porcentaje_participacion), 5);
    // pos 481–500 blanks

    buf.iter().collect()
}

// ── Importer ──────────────────────────────────────────────────────────────────

/// Parse an existing `.720` file back into editable structs.
pub fn import_from_bytes(bytes: &[u8]) -> Result<(Tipo1Fields, Vec<Tipo2Fields>), String> {
    let (text, _, had_errors) = WINDOWS_1252.decode(bytes);
    if had_errors {
        return Err("Codificación inválida (se esperaba ISO-8859-1)".to_string());
    }

    let raw: Vec<&str> = if text.contains("\r\n") {
        text.split("\r\n").collect()
    } else {
        text.split('\n').collect()
    };
    let lines: Vec<&str> = raw.iter().copied().filter(|l| !l.is_empty()).collect();

    if lines.is_empty() {
        return Err("Fichero vacío".to_string());
    }
    for (i, line) in lines.iter().enumerate() {
        if line.chars().count() != 500 {
            return Err(format!(
                "Línea {} tiene {} caracteres (se esperaban 500)",
                i + 1,
                line.chars().count()
            ));
        }
    }
    if fld(lines[0], 1, 1) != "1" {
        return Err("El primer registro no es tipo 1".to_string());
    }

    let t1 = parse_t1(lines[0]);
    let t2s = lines[1..]
        .iter()
        .filter(|l| fld(l, 1, 1) == "2")
        .map(|l| parse_t2(l))
        .collect();

    Ok((t1, t2s))
}

fn parse_t1(raw: &str) -> Tipo1Fields {
    Tipo1Fields {
        ejercicio: fld(raw, 5, 8),
        nif_declarante: fld(raw, 9, 17).trim().to_string(),
        nombre_declarante: fld(raw, 18, 57).trim().to_string(),
        nif_presentador: fld(raw, 59, 67).trim().to_string(),
        nombre_presentador: fld(raw, 68, 107).trim().to_string(),
        num_identificativo: fld(raw, 108, 120).trim().to_string(),
        dec_complementaria: ch(raw, 121) == 'C',
        dec_sustitutiva: ch(raw, 122) == 'S',
        num_dec_anterior: fld(raw, 123, 135).trim().to_string(),
    }
}

fn parse_t2(raw: &str) -> Tipo2Fields {
    let v1_sign = ch(raw, 432);
    let v1_cents: i64 = fld(raw, 433, 446).trim().parse().unwrap_or(0);
    let valoracion1 = if v1_sign == 'N' {
        -(v1_cents as f64 / 100.0)
    } else {
        v1_cents as f64 / 100.0
    };

    let v2_sign = ch(raw, 447);
    let v2_cents: i64 = fld(raw, 448, 461).trim().parse().unwrap_or(0);
    let valoracion2 = if v2_sign == 'N' {
        -(v2_cents as f64 / 100.0)
    } else {
        v2_cents as f64 / 100.0
    };

    Tipo2Fields {
        nif_declarado: fld(raw, 18, 26).trim().to_string(),
        nif_representante: fld(raw, 27, 35).trim().to_string(),
        nombre_declarado: fld(raw, 36, 75).trim().to_string(),
        clave_condicion: ch(raw, 76),
        clave_tipo_bien: ch(raw, 102),
        subclave: ch(raw, 103),
        codigo_pais: fld(raw, 129, 130),
        clave_identificacion: ch(raw, 131),
        identificacion_valores: fld(raw, 132, 143).trim().to_string(),
        clave_id_cuenta: ch(raw, 144),
        codigo_bic: fld(raw, 145, 155).trim().to_string(),
        identificacion_cuenta: fld(raw, 156, 189).trim().to_string(),
        identificacion_entidad: fld(raw, 190, 230).trim().to_string(),
        nif_fiscal_pais: fld(raw, 231, 250).trim().to_string(),
        domicilio: fld(raw, 251, 290).trim().to_string(),
        localidad: fld(raw, 343, 372).trim().to_string(),
        municipio: fld(raw, 373, 402).trim().to_string(),
        codigo_postal: fld(raw, 403, 412).trim().to_string(),
        pais_domicilio: fld(raw, 413, 414).trim().to_string(),
        fecha_incorporacion: fld(raw, 415, 422),
        origen: ch(raw, 423),
        fecha_extincion: fld(raw, 424, 431),
        valoracion1,
        valoracion2,
        clave_represent_valores: ch(raw, 462),
        num_valores: fld(raw, 463, 474).trim().parse().unwrap_or(0),
        clave_tipo_inmueble: ch(raw, 475),
        porcentaje_participacion: fld(raw, 476, 480).trim().parse().unwrap_or(0),
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Extract a 1-based inclusive substring.
fn fld(raw: &str, start: usize, end: usize) -> String {
    raw.chars().skip(start - 1).take(end - start + 1).collect()
}

/// Extract a single 1-based character.
fn ch(raw: &str, pos: usize) -> char {
    raw.chars().nth(pos - 1).unwrap_or(' ')
}

/// Write `s` into `buf` at 1-based position `start`, up to `width` chars.
/// Remaining slots within `width` are left as-is (spaces from the initial fill).
fn wstr(buf: &mut Vec<char>, start: usize, s: &str, width: usize) {
    for (i, c) in s.chars().take(width).enumerate() {
        buf[start - 1 + i] = c;
    }
}

/// Right-pad string with spaces to exactly `len` chars.
fn pad_right(s: &str, len: usize) -> String {
    let mut out: String = s.chars().take(len).collect();
    while out.chars().count() < len {
        out.push(' ');
    }
    out
}

/// Left-pad the numeric digits of `s` with zeros to `len` chars.
/// Non-digit characters are stripped first.
fn pad_left_zeros(s: &str, len: usize) -> String {
    let digits: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
    format!("{:0>width$}", &digits[digits.len().saturating_sub(len)..], width = len)
}

/// Ensure `s` is exactly 8 ASCII digits; returns "00000000" on mismatch.
fn norm_date(s: &str) -> String {
    if s.len() == 8 && s.chars().all(|c| c.is_ascii_digit()) {
        s.to_string()
    } else {
        "00000000".to_string()
    }
}

/// Encode `cents` as (sign_char, zero-padded-absolute-digits).
///
/// - sign_char: `' '` for non-negative, `'N'` for negative.
/// - digits: absolute value, zero-padded to `width` chars.
fn encode_amount(cents: i64, width: usize) -> (char, String) {
    let sign = if cents < 0 { 'N' } else { ' ' };
    let abs = cents.unsigned_abs();
    (sign, format!("{:0>width$}", abs, width = width))
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_t1() -> Tipo1Fields {
        Tipo1Fields {
            ejercicio: "2024".to_string(),
            nif_declarante: "12345678Z".to_string(),
            nombre_declarante: "JUAN GARCIA LOPEZ".to_string(),
            num_identificativo: "7200000000001".to_string(),
            ..Default::default()
        }
    }

    fn valid_t2() -> Tipo2Fields {
        Tipo2Fields {
            nif_declarado: "12345678Z".to_string(),
            nombre_declarado: "BANCO SUIZO SA".to_string(),
            clave_condicion: '1',
            clave_tipo_bien: 'C',
            subclave: '1',
            codigo_pais: "CH".to_string(),
            clave_identificacion: '0',
            clave_id_cuenta: 'I',
            codigo_bic: "UBSWCHZH80A".to_string(),
            identificacion_cuenta: "CH9300762011623852957".to_string(),
            identificacion_entidad: "UBS SWITZERLAND AG".to_string(),
            fecha_incorporacion: "20240101".to_string(),
            origen: 'A',
            fecha_extincion: "00000000".to_string(),
            valoracion1: 41446.27,
            porcentaje_participacion: 10000,
            pais_domicilio: "CH".to_string(),
            ..Default::default()
        }
    }

    #[test]
    fn record_length_is_500() {
        let t1 = valid_t1();
        let t2 = valid_t2();
        let bytes = generate_file(&t1, &[t2]);
        let text = String::from_utf8_lossy(&bytes);
        let lines: Vec<&str> = text
            .split("\r\n")
            .filter(|l| !l.is_empty())
            .collect();
        assert_eq!(lines.len(), 2, "should have 1 T1 + 1 T2 line");
        for (i, line) in lines.iter().enumerate() {
            assert_eq!(
                line.chars().count(),
                500,
                "line {} should be 500 chars, got {}",
                i + 1,
                line.chars().count()
            );
        }
    }

    #[test]
    fn totals_are_computed() {
        let t1 = valid_t1();
        let t2a = Tipo2Fields { valoracion1: 100.00, ..valid_t2() };
        let t2b = Tipo2Fields { valoracion1: 200.50, nif_declarado: "87654321X".to_string(), ..valid_t2() };
        let bytes = generate_file(&t1, &[t2a, t2b]);
        let text = String::from_utf8_lossy(&bytes);
        let t1_line: Vec<char> = text.lines().next().unwrap().chars().collect();
        // TOTAL_REGISTROS_T2 at pos 136–144
        let count: String = t1_line[135..144].iter().collect();
        assert_eq!(count, "000000002");
        // SUMA_VAL1 at pos 146–162 (29_050 cents = 300.50 €)
        let suma: String = t1_line[145..162].iter().collect();
        assert_eq!(suma, "00000000000030050");
    }

    #[test]
    fn roundtrip_import() {
        let t1 = valid_t1();
        let t2 = valid_t2();
        let bytes = generate_file(&t1, &[t2.clone()]);
        let (t1_back, t2s_back) = import_from_bytes(&bytes).unwrap();
        assert_eq!(t1_back.nif_declarante, t1.nif_declarante);
        assert_eq!(t1_back.nombre_declarante, t1.nombre_declarante);
        assert_eq!(t2s_back.len(), 1);
        let r = &t2s_back[0];
        assert_eq!(r.nif_declarado, t2.nif_declarado);
        assert_eq!(r.clave_tipo_bien, t2.clave_tipo_bien);
        assert!((r.valoracion1 - t2.valoracion1).abs() < 0.01);
    }

    #[test]
    fn encode_amount_negative() {
        let (sign, digits) = encode_amount(-15050, 14);
        assert_eq!(sign, 'N');
        assert_eq!(digits, "00000000015050");
    }

    #[test]
    fn pad_left_zeros_works() {
        assert_eq!(pad_left_zeros("42", 5), "00042");
        assert_eq!(pad_left_zeros("", 5), "00000");
        assert_eq!(pad_left_zeros("123456", 4), "3456"); // truncate from left
    }
}
