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
        <div class="max-w-4xl mx-auto px-4 py-12">
            // Navbar
            <div class="navbar bg-base-100 rounded-box shadow mb-8">
                <div class="flex-1 gap-2">
                    <div class="btn btn-ghost text-xl font-bold tracking-tight">
                        <span class="badge badge-primary badge-sm font-mono">"720"</span>
                        "Validador Modelo 720"
                    </div>
                </div>
                <div class="flex-none">
                    <div class="badge badge-ghost gap-1.5 py-3">
                        <span class="w-2 h-2 rounded-full bg-success animate-pulse"></span>
                        "100% local"
                    </div>
                </div>
            </div>

            // Hero / Drop Zone
            <DropZone on_file=on_file.clone() />

            // File name
            <Show when=move || !get_file_name.get().is_empty()>
                <div class="flex justify-center mt-3">
                    <div class="badge badge-outline badge-lg gap-2 font-mono text-sm">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                        </svg>
                        {move || get_file_name.get()}
                    </div>
                </div>
            </Show>

            // Loading
            <Show when=move || get_loading.get()>
                <div class="flex justify-center items-center gap-3 my-8">
                    <span class="loading loading-spinner loading-md text-primary"></span>
                    <span class="text-base-content/60">"Validando fichero..."</span>
                </div>
            </Show>

            // Results
            <Show when=move || get_result.get().is_some()>
                {move || {
                    let r = get_result.get().unwrap();
                    view! {
                        <div class="animate-in fade-in mt-6 space-y-6">
                            <StatusBadge is_valid=r.is_valid error_count=r.errors.len() warning_count=r.warnings.len() />
                            {r.summary.as_ref().map(|s| view! { <SummaryPanel summary=s.clone() /> })}
                            <ErrorList errors=r.errors.clone() />
                            <WarningList warnings=r.warnings.clone() />
                        </div>
                    }
                }}
            </Show>

            // Footer
            <footer class="mt-12 text-center text-base-content/40 text-xs">
                "Basado en Orden HAP/72/2013 — Tu fichero nunca sale de este navegador"
            </footer>
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
            class="relative card bg-base-100 shadow-xl border-2 border-dashed border-base-300 hover:border-primary transition-all duration-300 cursor-pointer group"
            on:drop=on_drop
            on:dragover=|ev: DragEvent| ev.prevent_default()
        >
            <div class="card-body items-center text-center py-12">
                <div class="w-16 h-16 rounded-2xl bg-primary/10 flex items-center justify-center mb-2 group-hover:-translate-y-1 transition-transform duration-300">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8 text-primary" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" />
                    </svg>
                </div>
                <h2 class="card-title text-lg">"Arrastra tu fichero .720 aqui"</h2>
                <p class="text-base-content/50 text-sm">"o pulsa para seleccionar desde tu equipo"</p>
                <div class="card-actions mt-4">
                    <div class="btn btn-primary btn-sm gap-2 pointer-events-none">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12" />
                        </svg>
                        "Seleccionar fichero"
                    </div>
                </div>
            </div>
            <input
                type="file"
                accept=".720,.txt"
                on:change=on_change
                class="absolute inset-0 w-full h-full opacity-0 cursor-pointer"
            />
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
fn StatusBadge(is_valid: bool, error_count: usize, warning_count: usize) -> impl IntoView {
    if is_valid {
        view! {
            <div class="alert alert-success shadow-lg">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
                <div>
                    <h3 class="font-bold">"Fichero valido"</h3>
                    <div class="text-xs opacity-80">
                        {if warning_count > 0 {
                            format!("Sin errores — {} avisos", warning_count)
                        } else {
                            "Sin errores ni avisos".to_string()
                        }}
                    </div>
                </div>
            </div>
        }.into_any()
    } else {
        let detail = format!("{} errores encontrados", error_count);
        view! {
            <div class="alert alert-error shadow-lg">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
                <div>
                    <h3 class="font-bold">"Fichero invalido"</h3>
                    <div class="text-xs opacity-80">{detail}</div>
                </div>
            </div>
        }.into_any()
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
    let t2_match = summary.total_registros_t2_declarado == summary.total_registros_t2_real;
    let t2_badge_class = if t2_match {
        "badge badge-success badge-sm"
    } else {
        "badge badge-error badge-sm"
    };

    view! {
        <div class="card bg-base-100 shadow-xl">
            <div class="card-body">
                <h2 class="card-title text-sm uppercase tracking-wider text-base-content/50 font-semibold mb-4">
                    "Resumen de la declaracion"
                </h2>
                <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                    // Ejercicio
                    <div class="stat bg-base-200 rounded-box p-4">
                        <div class="stat-title text-xs">"Ejercicio"</div>
                        <div class="stat-value text-2xl">{ejercicio}</div>
                    </div>
                    // NIF
                    <div class="stat bg-base-200 rounded-box p-4">
                        <div class="stat-title text-xs">"NIF Declarante"</div>
                        <div class="stat-value text-xl font-mono">{nif}</div>
                    </div>
                    // Nombre
                    <div class="sm:col-span-2 stat bg-base-200 rounded-box p-4">
                        <div class="stat-title text-xs">"Nombre / Razon Social"</div>
                        <div class="stat-value text-lg">{nombre}</div>
                    </div>
                </div>
                <div class="divider my-2"></div>
                <div class="grid grid-cols-2 sm:grid-cols-4 gap-4">
                    // T2 declarado
                    <div class="text-center">
                        <div class="text-xs text-base-content/50 mb-1">"T2 declarado"</div>
                        <div class="text-xl font-bold font-mono">{t2_decl}</div>
                    </div>
                    // T2 real
                    <div class="text-center">
                        <div class="text-xs text-base-content/50 mb-1">"T2 real"</div>
                        <div class="flex items-center justify-center gap-2">
                            <span class="text-xl font-bold font-mono">{t2_real}</span>
                            <span class=t2_badge_class>{if t2_match { "OK" } else { "!=" }}</span>
                        </div>
                    </div>
                    // Val1
                    <div class="text-center">
                        <div class="text-xs text-base-content/50 mb-1">"Valoracion 1"</div>
                        <div class="text-lg font-semibold font-mono">{val1}</div>
                    </div>
                    // Val2
                    <div class="text-center">
                        <div class="text-xs text-base-content/50 mb-1">"Valoracion 2"</div>
                        <div class="text-lg font-semibold font-mono">{val2}</div>
                    </div>
                </div>
            </div>
        </div>
    }
}

fn format_currency(val: f64) -> String {
    let abs = val.abs();
    let integer = abs as u64;
    let cents = ((abs - integer as f64) * 100.0).round() as u64;
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
    let count_str = format!("{}", errors.len());
    let rows: Vec<_> = errors.into_iter().map(|e| {
        let severity_class = match e.severity {
            Severity::Fatal => "bg-error/10",
            Severity::Error => "bg-warning/10",
            Severity::Warning => "bg-info/10",
        };
        let badge_class = match e.severity {
            Severity::Fatal => "badge badge-error badge-sm font-mono",
            Severity::Error => "badge badge-warning badge-sm font-mono",
            Severity::Warning => "badge badge-info badge-sm font-mono",
        };
        let code = e.code;
        let line = format!("{}", e.line);
        let field = e.field;
        let message = e.message;
        let row_class = format!("hover:bg-base-200 transition-colors {}", severity_class);
        view! {
            <tr class=row_class>
                <td><span class=badge_class>{code}</span></td>
                <td class="font-mono text-sm">{line}</td>
                <td class="font-mono text-xs text-base-content/50">{field}</td>
                <td class="text-sm">{message}</td>
            </tr>
        }
    }).collect();

    view! {
        <div class="card bg-base-100 shadow-xl">
            <div class="card-body p-0">
                <div class="flex items-center gap-3 px-6 pt-5 pb-3">
                    <h3 class="font-semibold">"Errores"</h3>
                    <span class="badge badge-error badge-sm">{count_str}</span>
                </div>
                <div class="overflow-x-auto">
                    <table class="table table-sm">
                        <thead>
                            <tr class="text-xs uppercase tracking-wider">
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
            </div>
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
        let line = format!("{}", e.line);
        let field = e.field;
        let message = e.message;
        view! {
            <tr class="hover:bg-base-200 transition-colors bg-warning/5">
                <td><span class="badge badge-warning badge-sm font-mono">{code}</span></td>
                <td class="font-mono text-sm">{line}</td>
                <td class="font-mono text-xs text-base-content/50">{field}</td>
                <td class="text-sm">{message}</td>
            </tr>
        }
    }).collect();

    view! {
        <div class="collapse collapse-arrow bg-base-100 shadow-xl">
            <input type="checkbox" />
            <div class="collapse-title font-semibold flex items-center gap-3">
                "Avisos"
                <span class="badge badge-warning badge-sm">{count_str}</span>
            </div>
            <div class="collapse-content p-0">
                <div class="overflow-x-auto">
                    <table class="table table-sm">
                        <thead>
                            <tr class="text-xs uppercase tracking-wider">
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
            </div>
        </div>
    }.into_any()
}
