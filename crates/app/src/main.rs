use leptos::prelude::*;
use leptos::*;
use validator720::{validate, Severity, ValidationError, ValidationResult};
use wasm_bindgen::prelude::*;
use web_sys::{DragEvent, Event, HtmlInputElement};

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let file_name = signal(String::new());
    let result = signal(None::<ValidationResult>);
    let loading = signal(false);

    let on_file = {
        let (_, set_file_name) = file_name;
        let (_, set_result) = result;
        let (_, set_loading) = loading;
        move |name: String, bytes: Vec<u8>| {
            set_file_name.set(name);
            set_loading.set(true);
            let r = validate(&bytes);
            set_result.set(Some(r));
            set_loading.set(false);
        }
    };

    let (get_file_name, _) = file_name;
    let (get_result, _) = result;
    let (get_loading, _) = loading;

    view! {
        <div class="header">
            <div class="header-icon">"720"</div>
            <h1>"Validador Modelo 720"</h1>
            <p>"Declaración sobre bienes y derechos en el extranjero"</p>
            <div class="privacy-note">
                <span class="privacy-dot"></span>
                "Tu fichero no sale de este navegador"
            </div>
        </div>

        <DropZone on_file=on_file.clone() />

        <Show when=move || !get_file_name.get().is_empty()>
            <p class="filename">
                <span>{move || get_file_name.get()}</span>
            </p>
        </Show>

        <Show when=move || get_loading.get()>
            <p class="loading">
                <span class="spinner"></span>
                "Validando..."
            </p>
        </Show>

        <Show when=move || get_result.get().is_some()>
            {move || {
                let r = get_result.get().unwrap();
                view! {
                    <div class="fade-in">
                        <StatusBadge is_valid=r.is_valid error_count=r.errors.len() />
                        {r.summary.as_ref().map(|s| view! { <SummaryPanel summary=s.clone() /> })}
                        <ErrorList errors=r.errors.clone() />
                        <WarningList warnings=r.warnings.clone() />
                    </div>
                }
            }}
        </Show>

        <div class="footer">
            "Validación local basada en Orden HAP/72/2013"
        </div>
    }
}

#[component]
fn DropZone(on_file: impl Fn(String, Vec<u8>) + Clone + 'static) -> impl IntoView {
    let on_file_drop = on_file.clone();
    let on_file_input = on_file;

    let on_drop = move |ev: DragEvent| {
        ev.prevent_default();
        if let Some(dt) = ev.data_transfer() {
            if let Some(files) = dt.files() {
                if let Some(file) = files.get(0) {
                    let name = file.name();
                    let on_file = on_file_drop.clone();
                    read_file(file, move |bytes| {
                        on_file(name.clone(), bytes);
                    });
                }
            }
        }
    };

    let on_change = move |ev: Event| {
        let target: HtmlInputElement = ev.target().unwrap().unchecked_into();
        if let Some(files) = target.files() {
            if let Some(file) = files.get(0) {
                let name = file.name();
                let on_file = on_file_input.clone();
                read_file(file, move |bytes| {
                    on_file(name.clone(), bytes);
                });
            }
        }
    };

    view! {
        <div
            class="dropzone"
            on:drop=on_drop
            on:dragover=|ev: DragEvent| ev.prevent_default()
        >
            <div class="dropzone-icon">"^"</div>
            <h3>"Arrastra tu fichero .720 aqui"</h3>
            <p>"o pulsa para seleccionar desde tu equipo"</p>
            <div class="file-btn">"Seleccionar fichero"</div>
            <input type="file" accept=".720,.txt" on:change=on_change />
        </div>
    }
}

fn read_file(file: web_sys::File, callback: impl FnOnce(Vec<u8>) + 'static) {
    use gloo_file::callbacks::read_as_bytes;
    use gloo_file::File as GlooFile;

    let gloo_file = GlooFile::from(file);
    read_as_bytes(&gloo_file, move |result| {
        if let Ok(bytes) = result {
            callback(bytes);
        }
    });
}

#[component]
fn StatusBadge(is_valid: bool, error_count: usize) -> impl IntoView {
    let text = if is_valid {
        "VALIDO".to_string()
    } else {
        format!("INVALIDO  —  {} errores", error_count)
    };
    let class = if is_valid { "badge badge-valid" } else { "badge badge-invalid" };
    view! {
        <div class="status-bar">
            <span class=class>{text}</span>
        </div>
    }
}

#[component]
fn SummaryPanel(summary: validator720::FileSummary) -> impl IntoView {
    let ejercicio = summary.ejercicio.clone();
    let nif = summary.nif_declarante.clone();
    let nombre = summary.nombre_declarante.clone();
    let t2_decl = format!("{}", summary.total_registros_t2_declarado);
    let t2_real = format!("{}", summary.total_registros_t2_real);
    let val1 = format_currency(summary.suma_val1);
    let val2 = format_currency(summary.suma_val2);

    view! {
        <div class="summary">
            <div class="summary-title">"Resumen de la declaracion"</div>
            <div class="summary-grid">
                <div class="summary-item">
                    <span class="summary-label">"Ejercicio"</span>
                    <span class="summary-value">{ejercicio}</span>
                </div>
                <div class="summary-item">
                    <span class="summary-label">"NIF Declarante"</span>
                    <span class="summary-value mono">{nif}</span>
                </div>
                <div class="summary-item full-width">
                    <span class="summary-label">"Nombre / Razon Social"</span>
                    <span class="summary-value">{nombre}</span>
                </div>
                <div class="summary-item">
                    <span class="summary-label">"Registros T2 (declarado)"</span>
                    <span class="summary-value">{t2_decl}</span>
                </div>
                <div class="summary-item">
                    <span class="summary-label">"Registros T2 (real)"</span>
                    <span class="summary-value">{t2_real}</span>
                </div>
                <div class="summary-item">
                    <span class="summary-label">"Suma Valoracion 1"</span>
                    <span class="summary-value mono">{val1}</span>
                </div>
                <div class="summary-item">
                    <span class="summary-label">"Suma Valoracion 2"</span>
                    <span class="summary-value mono">{val2}</span>
                </div>
            </div>
        </div>
    }
}

fn format_currency(val: f64) -> String {
    let abs = val.abs();
    let integer = abs as u64;
    let cents = ((abs - integer as f64) * 100.0).round() as u64;

    // Format with thousand separators
    let int_str = integer.to_string();
    let mut formatted = String::new();
    for (i, ch) in int_str.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            formatted.push('.');
        }
        formatted.push(ch);
    }
    let formatted: String = formatted.chars().rev().collect();

    let sign = if val < 0.0 { "-" } else { "" };
    format!("{}{},{:02} EUR", sign, formatted, cents)
}

#[component]
fn ErrorList(errors: Vec<ValidationError>) -> impl IntoView {
    if errors.is_empty() {
        return view! { <div></div> }.into_any();
    }
    let count = errors.len();
    let count_str = format!("{}", count);
    let rows: Vec<_> = errors.into_iter().map(|e| {
        let class = match e.severity {
            Severity::Fatal => "fatal",
            Severity::Error => "error",
            Severity::Warning => "warning",
        };
        let code = e.code;
        let line = e.line;
        let field = e.field;
        let message = e.message;
        view! {
            <tr class=class>
                <td>{code}</td>
                <td>{line}</td>
                <td>{field}</td>
                <td>{message}</td>
            </tr>
        }
    }).collect();
    view! {
        <div class="section-header">
            <h3>"Errores"</h3>
            <span class="section-count errors">{count_str}</span>
        </div>
        <div class="error-table-wrap">
            <table class="error-table">
                <thead>
                    <tr>
                        <th>"Codigo"</th>
                        <th>"Linea"</th>
                        <th>"Campo"</th>
                        <th>"Descripcion"</th>
                    </tr>
                </thead>
                <tbody>
                    {rows}
                </tbody>
            </table>
        </div>
    }.into_any()
}

#[component]
fn WarningList(warnings: Vec<ValidationError>) -> impl IntoView {
    if warnings.is_empty() {
        return view! { <div></div> }.into_any();
    }
    let count_str = format!("{}", warnings.len());
    let rows: Vec<_> = warnings.into_iter().map(|e| {
        let code = e.code;
        let line = e.line;
        let field = e.field;
        let message = e.message;
        view! {
            <tr class="warning">
                <td>{code}</td>
                <td>{line}</td>
                <td>{field}</td>
                <td>{message}</td>
            </tr>
        }
    }).collect();
    view! {
        <details class="collapsible">
            <summary>
                <span class="chevron">">"</span>
                <h3>"Avisos"</h3>
                <span class="section-count warnings">{count_str}</span>
            </summary>
            <div class="error-table-wrap">
                <table class="error-table">
                    <thead>
                        <tr>
                            <th>"Codigo"</th>
                            <th>"Linea"</th>
                            <th>"Campo"</th>
                            <th>"Descripcion"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {rows}
                    </tbody>
                </table>
            </div>
        </details>
    }.into_any()
}
