#!/usr/bin/env python3
"""
Generate test fixture files for Modelo 720 validator.
Files are encoded in Windows-1252/ISO-8859-1.
Each record is 500 characters + CRLF (502 bytes total).
"""

import os

ENCODING = "windows-1252"
RECORD_LEN = 500


def pad_right(s, length):
    """Left-align string, pad with spaces on right."""
    return s[:length].ljust(length)


def pad_left(s, length):
    """Right-align string, pad with zeros on left."""
    return s[:length].zfill(length)


def make_t1_record(
    ejercicio,
    nif_declarante,
    nombre,
    telefono,
    contacto,
    num_identificativo,
    dec_complementaria,
    dec_sustitutiva,
    num_dec_anterior,
    total_t2,
    signo_val1,
    suma_val1,
    signo_val2,
    suma_val2,
):
    """Build a Type 1 (T1) record of exactly 500 chars."""
    rec = ""
    rec += "1"                                      # pos 1
    rec += "720"                                     # pos 2-4
    rec += ejercicio                                 # pos 5-8 (4 chars)
    rec += pad_right(nif_declarante, 9)              # pos 9-17
    rec += pad_right(nombre, 40)                     # pos 18-57
    rec += "T"                                       # pos 58
    rec += pad_right(telefono, 9)                    # pos 59-67
    rec += pad_right(contacto, 40)                   # pos 68-107
    rec += pad_right(num_identificativo, 13)         # pos 108-120
    rec += dec_complementaria                        # pos 121 (1 char)
    rec += dec_sustitutiva                           # pos 122 (1 char)
    rec += pad_left(num_dec_anterior, 13)            # pos 123-135
    rec += pad_left(str(total_t2), 9)                # pos 136-144
    rec += signo_val1                                # pos 145 (1 char)
    rec += pad_left(str(suma_val1), 17)              # pos 146-162
    rec += signo_val2                                # pos 163 (1 char)
    rec += pad_left(str(suma_val2), 17)              # pos 164-180
    rec += " " * 320                                 # pos 181-500

    assert len(rec) == RECORD_LEN, f"T1 record length {len(rec)} != {RECORD_LEN}"
    return rec


def make_t2_record_cuenta(
    ejercicio,
    nif_declarante,
    nif_declarado,
    nif_rep_legal,
    nombre_declarado,
    clave_condicion,
    clave_tipo_bien,
    subclave,
    codigo_pais,
    clave_identificacion,
    clave_id_cuenta,
    codigo_bic,
    codigo_cuenta,
    identificacion_entidad,
    nif_pais_residencia,
    nombre_via,
    poblacion,
    provincia,
    cod_postal,
    codigo_pais_domicilio,
    fecha_incorporacion,
    origen,
    fecha_extincion,
    signo_val1,
    valoracion1,
    signo_val2,
    valoracion2,
):
    """Build a Type 2 (T2) record for a 'C' (cuenta) bien, exactly 500 chars."""
    rec = ""
    rec += "2"                                       # pos 1
    rec += "720"                                     # pos 2-4
    rec += ejercicio                                 # pos 5-8
    rec += pad_right(nif_declarante, 9)              # pos 9-17
    rec += pad_right(nif_declarado, 9)               # pos 18-26
    rec += pad_right(nif_rep_legal, 9)               # pos 27-35
    rec += pad_right(nombre_declarado, 40)           # pos 36-75
    rec += clave_condicion                           # pos 76 (1 char)
    rec += " " * 25                                  # pos 77-101 tipo titularidad
    rec += clave_tipo_bien                           # pos 102 (1 char)
    rec += subclave                                  # pos 103 (1 char)
    rec += " " * 25                                  # pos 104-128 tipo derecho real
    rec += pad_right(codigo_pais, 2)                 # pos 129-130
    rec += clave_identificacion                      # pos 131 (1 char)
    rec += " " * 12                                  # pos 132-143 identificacion valores (C)
    rec += clave_id_cuenta                           # pos 144 (1 char)
    rec += pad_right(codigo_bic, 11)                 # pos 145-155
    rec += pad_right(codigo_cuenta, 34)              # pos 156-189
    rec += pad_right(identificacion_entidad, 41)     # pos 190-230
    rec += pad_right(nif_pais_residencia, 20)        # pos 231-250
    rec += pad_right(nombre_via, 52)                 # pos 251-302
    rec += " " * 40                                  # pos 303-342 complemento
    rec += pad_right(poblacion, 30)                  # pos 343-372
    rec += pad_right(provincia, 30)                  # pos 373-402
    rec += pad_right(cod_postal, 10)                 # pos 403-412
    rec += pad_right(codigo_pais_domicilio, 2)       # pos 413-414
    rec += fecha_incorporacion                       # pos 415-422 (8 chars)
    rec += origen                                    # pos 423 (1 char)
    rec += fecha_extincion                           # pos 424-431 (8 chars)
    rec += signo_val1                                # pos 432 (1 char)
    rec += pad_left(str(valoracion1), 14)            # pos 433-446
    rec += signo_val2                                # pos 447 (1 char)
    rec += pad_left(str(valoracion2), 14)            # pos 448-461
    rec += " "                                       # pos 462 clave represent valores (C)
    rec += "0" * 12                                  # pos 463-474 num valores (C)
    rec += " "                                       # pos 475 clave tipo inmueble (C)
    rec += "10000"                                   # pos 476-480 porcentaje participacion
    rec += " " * 20                                  # pos 481-500 blancos

    assert len(rec) == RECORD_LEN, f"T2 record length {len(rec)} != {RECORD_LEN}"
    return rec


def write_file(path, records):
    """Write records to a binary file encoded in windows-1252 with CRLF line endings."""
    with open(path, "wb") as f:
        for rec in records:
            encoded = rec.encode(ENCODING)
            f.write(encoded + b"\r\n")


def generate_valid():
    """Generate a valid Modelo 720 file with 1 T1 + 2 T2 records."""
    t2_r1 = make_t2_record_cuenta(
        ejercicio="2024",
        nif_declarante="12345678Z",
        nif_declarado="12345678Z",
        nif_rep_legal="         ",
        nombre_declarado="BANCO SUIZO SA",
        clave_condicion="1",
        clave_tipo_bien="C",
        subclave="1",
        codigo_pais="CH",
        clave_identificacion="0",
        clave_id_cuenta="I",
        codigo_bic="UBSWCHZH80A",
        codigo_cuenta="CH9300762011623852957",
        identificacion_entidad="UBS SWITZERLAND AG",
        nif_pais_residencia="CHE-123.456.789",
        nombre_via="BAHNHOFSTRASSE 45",
        poblacion="ZURICH",
        provincia="ZURICH",
        cod_postal="8001",
        codigo_pais_domicilio="CH",
        fecha_incorporacion="20240101",
        origen="A",
        fecha_extincion="00000000",
        signo_val1=" ",
        valoracion1=4144627,
        signo_val2=" ",
        valoracion2=0,
    )

    t2_r2 = make_t2_record_cuenta(
        ejercicio="2024",
        nif_declarante="12345678Z",
        nif_declarado="00000000T",
        nif_rep_legal="         ",
        nombre_declarado="CREDIT SUISSE AG",
        clave_condicion="1",
        clave_tipo_bien="C",
        subclave="1",
        codigo_pais="CH",
        clave_identificacion="0",
        clave_id_cuenta="I",
        codigo_bic="CRESCHZZ80A",
        codigo_cuenta="CH5604835012345678009",
        identificacion_entidad="CREDIT SUISSE AG",
        nif_pais_residencia="CHE-987.654.321",
        nombre_via="PARADEPLATZ 8",
        poblacion="ZURICH",
        provincia="ZURICH",
        cod_postal="8070",
        codigo_pais_domicilio="CH",
        fecha_incorporacion="20240101",
        origen="A",
        fecha_extincion="00000000",
        signo_val1=" ",
        valoracion1=2000000,
        signo_val2=" ",
        valoracion2=0,
    )

    t1 = make_t1_record(
        ejercicio="2024",
        nif_declarante="12345678Z",
        nombre="JUAN GARCIA LOPEZ",
        telefono="612345678",
        contacto="JUAN GARCIA LOPEZ",
        num_identificativo="7200000000001",
        dec_complementaria=" ",
        dec_sustitutiva=" ",
        num_dec_anterior="0",
        total_t2=2,
        signo_val1=" ",
        suma_val1=6144627,
        signo_val2=" ",
        suma_val2=0,
    )

    return [t1, t2_r1, t2_r2]


def generate_invalid():
    """Generate an invalid Modelo 720 file with intentional errors."""
    # Error 1: invalid NIF declarante (XXXXXXXXX)
    # Error 2: total_t2=5 but only 1 T2 record
    # Error 3: date '20241301' (month 13, invalid)

    t2_bad = make_t2_record_cuenta(
        ejercicio="2024",
        nif_declarante="XXXXXXXXX",   # invalid NIF
        nif_declarado="12345678Z",
        nif_rep_legal="         ",
        nombre_declarado="BANCO MALO SA",
        clave_condicion="1",
        clave_tipo_bien="C",
        subclave="1",
        codigo_pais="CH",
        clave_identificacion="0",
        clave_id_cuenta="I",
        codigo_bic="UBSWCHZH80A",
        codigo_cuenta="CH9300762011623852957",
        identificacion_entidad="UBS SWITZERLAND AG",
        nif_pais_residencia="CHE-123.456.789",
        nombre_via="BAHNHOFSTRASSE 45",
        poblacion="ZURICH",
        provincia="ZURICH",
        cod_postal="8001",
        codigo_pais_domicilio="CH",
        fecha_incorporacion="20241301",  # invalid date: month 13
        origen="A",
        fecha_extincion="00000000",
        signo_val1=" ",
        valoracion1=4144627,
        signo_val2=" ",
        valoracion2=0,
    )

    t1_bad = make_t1_record(
        ejercicio="2024",
        nif_declarante="XXXXXXXXX",   # invalid NIF
        nombre="TITULAR INVALIDO",
        telefono="612345678",
        contacto="TITULAR INVALIDO",
        num_identificativo="7200000000002",
        dec_complementaria=" ",
        dec_sustitutiva=" ",
        num_dec_anterior="0",
        total_t2=5,            # wrong count: says 5, but only 1 T2 record follows
        signo_val1=" ",
        suma_val1=4144627,
        signo_val2=" ",
        suma_val2=0,
    )

    return [t1_bad, t2_bad]


def verify_file(path):
    """Read file and verify each line is exactly 502 bytes (500 chars + CRLF)."""
    with open(path, "rb") as f:
        content = f.read()

    lines = content.split(b"\r\n")
    # Last element after final CRLF is empty string
    if lines and lines[-1] == b"":
        lines = lines[:-1]

    errors = []
    for i, line in enumerate(lines, 1):
        if len(line) != RECORD_LEN:
            errors.append(f"  Line {i}: {len(line)} chars (expected {RECORD_LEN})")

    if errors:
        print(f"ERRORS in {path}:")
        for e in errors:
            print(e)
    else:
        print(f"OK: {path} - {len(lines)} records, each {RECORD_LEN} chars + CRLF")


if __name__ == "__main__":
    fixtures_dir = os.path.join(os.path.dirname(__file__), "fixtures")
    os.makedirs(fixtures_dir, exist_ok=True)

    valid_path = os.path.join(fixtures_dir, "valid.720")
    invalid_path = os.path.join(fixtures_dir, "invalid.720")

    valid_records = generate_valid()
    write_file(valid_path, valid_records)
    print(f"Written: {valid_path}")

    invalid_records = generate_invalid()
    write_file(invalid_path, invalid_records)
    print(f"Written: {invalid_path}")

    print()
    verify_file(valid_path)
    verify_file(invalid_path)
