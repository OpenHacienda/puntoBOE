use crate::ValidationError;
use crate::nif;
use crate::parser::*;

pub fn validate_tipo1(record: &Record, errors: &mut Vec<ValidationError>) {
    let raw = &record.raw;
    let line = record.line;

    // E101: TIPO_SOPORTE (pos 58) must be 'T'
    let tipo_soporte = char_at(raw, 58);
    if tipo_soporte != 'T' {
        errors.push(ValidationError::error_pos(
            "E101",
            line,
            (58, 58),
            "TIPO_SOPORTE",
            &format!("TIPO_SOPORTE debe ser 'T', encontrado '{}'", tipo_soporte),
        ));
    }

    // E102: NIF_DECLARANTE (pos 9-17) must be valid
    let nif_declarante = field(raw, 9, 17);
    if !nif::validate_nif(&nif_declarante) {
        errors.push(ValidationError::error_pos(
            "E102",
            line,
            (9, 17),
            "NIF_DECLARANTE",
            &format!("NIF_DECLARANTE inválido: '{}'", nif_declarante.trim()),
        ));
    }

    // E103: APELLIDOS_NOMBRE (pos 18-57) must not be empty
    let nombre = field(raw, 18, 57);
    if nombre.trim().is_empty() {
        errors.push(ValidationError::error_pos(
            "E103",
            line,
            (18, 57),
            "APELLIDOS_NOMBRE",
            "APELLIDOS_NOMBRE vacío",
        ));
    }

    // E104: NUM_IDENTIFICATIVO (pos 108-120) must start with "720"
    let num_ident = field(raw, 108, 120);
    if !num_ident.starts_with("720") {
        errors.push(ValidationError::error_pos(
            "E104",
            line,
            (108, 120),
            "NUM_IDENTIFICATIVO",
            &format!(
                "NUM_IDENTIFICATIVO no empieza por '720': '{}'",
                num_ident.trim()
            ),
        ));
    }

    // E105: DEC_COMPLEMENTARIA (pos 121) and DEC_SUSTITUTIVA (pos 122) not both informed
    let dec_comp = char_at(raw, 121);
    let dec_sust = char_at(raw, 122);
    if dec_comp == 'C' && dec_sust == 'S' {
        errors.push(ValidationError::error_pos(
            "E105",
            line,
            (121, 122),
            "DEC_COMPLEMENTARIA/DEC_SUSTITUTIVA",
            "DEC_COMPLEMENTARIA y DEC_SUSTITUTIVA ambas informadas",
        ));
    }

    // E106: DEC_SUSTITUTIVA='S' but NUM_DEC_ANTERIOR is zeros
    let num_dec_anterior = field(raw, 123, 135);
    if dec_sust == 'S' && is_zeros(&num_dec_anterior) {
        errors.push(ValidationError::error_pos(
            "E106",
            line,
            (123, 135),
            "NUM_DEC_ANTERIOR",
            "DEC_SUSTITUTIVA='S' pero NUM_DEC_ANTERIOR es ceros",
        ));
    }

    // E110: BLANCOS pos 181-500 must be all spaces
    let blancos = field(raw, 181, 500);
    if !is_blank(&blancos) {
        errors.push(ValidationError::error_pos(
            "E110",
            line,
            (181, 500),
            "BLANCOS",
            "Posiciones 181-500 contienen caracteres no blancos",
        ));
    }
}
