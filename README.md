# puntoBOE — Validador de ficheros BOE

Validador de ficheros de declaraciones tributarias en formato BOE (Boletín Oficial del Estado) de la AEAT. Funciona 100% en el navegador — sin backend, sin envío de datos a ningún servidor.

## Modelos soportados

| Modelo | Descripción | Estado |
|--------|-------------|--------|
| **720** | Declaración sobre bienes y derechos situados en el extranjero | Implementado |

> Se prevé añadir más modelos en el futuro (349, 347, 190, etc.).

## Cómo funciona

1. El usuario arrastra o selecciona un fichero `.720` en el navegador
2. El fichero se lee localmente con WebAssembly — **nunca sale del navegador**
3. Se validan estructura, campos, NIF/NIE/CIF, fechas, importes, y totales cruzados
4. Se muestran errores, avisos, y un resumen de la declaración

## Validaciones implementadas (Modelo 720)

Basado en la **Orden HAP/72/2013** y el diseño de registro oficial de la AEAT.

- **Estructurales** (E001–E007) — codificación ISO-8859-1, longitud 500 caracteres, tipo de registro, modelo, ejercicio
- **Registro tipo 1** (E101–E110) — NIF declarante, tipo soporte, número identificativo, declaración complementaria/sustitutiva, blancos
- **Registro tipo 2** (E201–E233) — NIF declarado, clave de bien (C/V/I/S/B), subclave, país ISO-3166, ISIN, IBAN, fechas, valoraciones, porcentaje de participación
- **Cruzadas** (E107–E109) — total de registros T2, suma de valoraciones 1 y 2
- **Avisos** (W001–W005) — fecha futura, valoración elevada, porcentaje cero, BIC incorrecto, ISIN duplicado

## Stack

- **Rust** — lógica de validación (crate `validator720`, sin dependencias WASM)
- **Leptos 0.7** — UI reactiva compilada a WebAssembly (CSR)
- **Tailwind CSS + DaisyUI** — diseño de componentes
- **trunk** — bundler que compila Leptos a WASM y sirve estático
- **Nix** con `numtide/blueprint` — entorno de desarrollo y build reproducible

## Estructura del proyecto

```
├── flake.nix                        # Nix flake (numtide/blueprint)
├── Cargo.toml                       # Workspace
├── crates/
│   ├── validator/                   # Lógica pura (cargo test, sin WASM)
│   │   └── src/
│   │       ├── lib.rs               # API pública: validate(bytes) → ValidationResult
│   │       ├── parser.rs            # Parsing posiciones fijas ISO-8859-1
│   │       ├── nif.rs               # Validación NIF/NIE/CIF
│   │       ├── tipo1.rs             # Validaciones registro tipo 1
│   │       ├── tipo2.rs             # Validaciones registro tipo 2
│   │       └── cross.rs             # Validaciones cruzadas (totales, NIF)
│   └── app/                         # UI Leptos (compila a WASM)
│       ├── index.html               # Punto de entrada trunk
│       └── src/main.rs              # Componentes: DropZone, Summary, ErrorList
├── tests/fixtures/
│   ├── valid.720                    # Fixture válido (2 registros T2)
│   └── invalid.720                  # Fixture con errores intencionados
└── nix/
    ├── packages/default.nix         # nix build
    └── devshells/default.nix        # nix develop
```

## Desarrollo

```bash
# Entrar al entorno (requiere Nix con flakes)
nix develop

# Dev server con hot-reload
trunk serve

# Tests del validador
cargo test -p validator720

# Build de producción
trunk build --release
```

## Privacidad

Tu fichero **nunca sale de tu navegador**. Toda la validación ocurre en WebAssembly localmente. No hay backend, no hay cookies, no hay analytics. Puedes verificarlo desconectándote de internet y usando la aplicación offline.

## Referencia

- [Orden HAP/72/2013](https://www.boe.es/buscar/act.php?id=BOE-A-2013-842) — regulación del Modelo 720
- [Diseño de registro oficial (PDF)](https://sede.agenciatributaria.gob.es/static_files/Sede/Disenyo_registro/DR_Resto_Mod/archivos/modelo_720.pdf)

## Un proyecto de OpenHacienda

**puntoBOE** forma parte de [OpenHacienda](https://github.com/OpenHacienda), una organización que desarrolla herramientas abiertas para interactuar con la administración pública española.

Otros proyectos:

- [**llave**](https://github.com/OpenHacienda/llave) — Implementación open-source del sistema Cl@ve PIN / Cl@ve Móvil. CLI y clientes Flutter sobre un núcleo Rust compartido.
- [**website**](https://github.com/OpenHacienda/website) — Documentación y página del proyecto.

## Licencia

MIT
