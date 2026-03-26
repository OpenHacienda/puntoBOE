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
        <h1>"Validador Modelo 720"</h1>
        <p class="subtitle">"Validación 100% en navegador — tu fichero no sale de aquí"</p>
        <DropZone on_file=on_file.clone() />
        <Show when=move || !get_file_name.get().is_empty()>
            <p class="filename">{move || get_file_name.get()}</p>
        </Show>
        <Show when=move || get_loading.get()>
            <p class="loading">"Validando..."</p>
        </Show>
        <Show when=move || get_result.get().is_some()>
            {move || {
                let r = get_result.get().unwrap();
                view! {
                    <StatusBadge is_valid=r.is_valid error_count=r.errors.len() />
                    {r.summary.as_ref().map(|s| view! { <SummaryPanel summary=s.clone() /> })}
                    <ErrorList errors=r.errors.clone() />
                    <WarningList warnings=r.warnings.clone() />
                }
            }}
        </Show>
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
            <p>"Arrastra tu fichero .720 aquí"</p>
            <small>"o haz clic para seleccionar"</small>
            <br />
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
        "VÁLIDO ✓".to_string()
    } else {
        format!("INVÁLIDO ✗ ({} errores)", error_count)
    };
    let class = if is_valid { "badge badge-valid" } else { "badge badge-invalid" };
    view! { <span class=class>{text}</span> }
}

#[component]
fn SummaryPanel(summary: validator720::FileSummary) -> impl IntoView {
    let ejercicio = summary.ejercicio.clone();
    let nif = summary.nif_declarante.clone();
    let nombre = summary.nombre_declarante.clone();
    let t2_decl = summary.total_registros_t2_declarado;
    let t2_real = summary.total_registros_t2_real;
    let val1 = format!("{:.2} €", summary.suma_val1);
    let val2 = format!("{:.2} €", summary.suma_val2);

    view! {
        <div class="summary">
            <dl>
                <dt>"Ejercicio"</dt>
                <dd>{ejercicio}</dd>
                <dt>"NIF Declarante"</dt>
                <dd>{nif}</dd>
                <dt>"Nombre"</dt>
                <dd>{nombre}</dd>
                <dt>"Registros T2 (declarado)"</dt>
                <dd>{t2_decl}</dd>
                <dt>"Registros T2 (real)"</dt>
                <dd>{t2_real}</dd>
                <dt>"Suma Valoración 1"</dt>
                <dd>{val1}</dd>
                <dt>"Suma Valoración 2"</dt>
                <dd>{val2}</dd>
            </dl>
        </div>
    }
}

#[component]
fn ErrorList(errors: Vec<ValidationError>) -> impl IntoView {
    if errors.is_empty() {
        return view! { <p></p> }.into_any();
    }
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
        <h3 style="margin-top: 1rem;">"Errores"</h3>
        <table class="error-table">
            <thead>
                <tr>
                    <th>"Código"</th>
                    <th>"Línea"</th>
                    <th>"Campo"</th>
                    <th>"Descripción"</th>
                </tr>
            </thead>
            <tbody>
                {rows}
            </tbody>
        </table>
    }.into_any()
}

#[component]
fn WarningList(warnings: Vec<ValidationError>) -> impl IntoView {
    if warnings.is_empty() {
        return view! { <p></p> }.into_any();
    }
    let count = format!("Avisos ({})", warnings.len());
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
            <summary>{count}</summary>
            <table class="error-table">
                <thead>
                    <tr>
                        <th>"Código"</th>
                        <th>"Línea"</th>
                        <th>"Campo"</th>
                        <th>"Descripción"</th>
                    </tr>
                </thead>
                <tbody>
                    {rows}
                </tbody>
            </table>
        </details>
    }.into_any()
}
