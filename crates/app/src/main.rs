mod editor;

use leptos::prelude::*;
use leptos::*;
use leptos_fluent::{leptos_fluent, move_tr, tr};
use leptos_router::{
    components::{Route, Router, Routes, A},
    hooks::use_location,
    path,
};
use validator720::{Severity, T2Detail, ValidationError, ValidationResult, validate};
use wasm_bindgen::prelude::*;
use web_sys::{DragEvent, Event, HtmlInputElement};

#[wasm_bindgen(inline_js = "
const EUR_FMT = new Intl.NumberFormat('es-ES', { style: 'currency', currency: 'EUR' });
export function format_eur(v) { return EUR_FMT.format(v); }

export function trigger_download(bytes, filename) {
    const blob = new Blob([bytes], { type: 'application/octet-stream' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = filename;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    setTimeout(() => URL.revokeObjectURL(url), 1000);
}
")]
extern "C" {
    pub(crate) fn format_eur(v: f64) -> String;
    pub(crate) fn trigger_download(bytes: &[u8], filename: &str);
}

fn main() {
    mount_to_body(App);
}

// ── I18n provider ─────────────────────────────────────────────────────────────

#[component]
fn I18nProvider(children: Children) -> impl IntoView {
    leptos_fluent! {
        children: children(),
        locales: "./locales",
        default_language: "es",
        #[cfg(debug_assertions)]
        check_translations: "./src/**/*.rs",
    }
}

// ── App ───────────────────────────────────────────────────────────────────────

#[component]
fn App() -> impl IntoView {
    view! {
        <I18nProvider>
            <Router>
                <Layout />
            </Router>
        </I18nProvider>
    }
}

// ── Shared layout (navbar + route outlet) ─────────────────────────────────────

#[component]
fn Layout() -> impl IntoView {
    view! {
        <div class="max-w-4xl mx-auto px-4 py-12">
            // ── Navbar ────────────────────────────────────────────────────────
            <div class="navbar bg-base-100 rounded-box shadow mb-8">
                <div class="flex-1 gap-2">
                    <div class="btn btn-ghost text-xl font-bold tracking-tight">
                        <span class="badge badge-primary badge-sm font-mono">"720"</span>
                        "puntoBOE"
                    </div>
                    // Navigation tabs — active class derived from current path
                    {move || {
                        let loc = use_location();
                        let path = loc.pathname.get();
                        let cls_validate = if path == "/" { "tab tab-active" } else { "tab" };
                        let cls_editor   = if path == "/editor" { "tab tab-active" } else { "tab" };
                        view! {
                            <div class="tabs tabs-boxed bg-base-200 ml-4">
                                <A href="/" attr:class=cls_validate>
                                    {move_tr!("nav-validate")}
                                </A>
                                <A href="/editor" attr:class=cls_editor>
                                    {move_tr!("nav-create")}
                                </A>
                            </div>
                        }
                    }}
                </div>
                <div class="flex-none">
                    <div class="badge badge-ghost gap-1.5 py-3">
                        <span class="w-2 h-2 rounded-full bg-success animate-pulse"></span>
                        {move_tr!("nav-local")}
                    </div>
                </div>
            </div>

            // ── Route content ─────────────────────────────────────────────────
            <Routes fallback=|| view! { <p class="text-center opacity-40">"Página no encontrada"</p> }>
                <Route path=path!("/") view=ValidatorPage />
                <Route path=path!("/editor") view=editor::EditorPage />
            </Routes>

            // ── Footer ────────────────────────────────────────────────────────
            <footer class="mt-12 text-center text-base-content/40 text-xs">
                {move_tr!("footer")}
            </footer>
        </div>
    }
}

// ── Validator page ────────────────────────────────────────────────────────────

#[component]
fn ValidatorPage() -> impl IntoView {
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
            leptos::task::spawn_local(async move {
                let r = validate(&bytes);
                set_result.set(Some(r));
                set_loading.set(false);
            });
        }
    };

    let (get_file_name, _) = file_name;
    let (get_result, _) = result;
    let (get_loading, _) = loading;

    view! {
        <div>
            <DropZone on_file=on_file.clone() file_name=get_file_name.into() />

            <Show when=move || get_loading.get()>
                <div class="flex justify-center items-center gap-3 my-8">
                    <span class="loading loading-spinner loading-md text-primary"></span>
                    <span class="text-base-content/60">{move_tr!("loading")}</span>
                </div>
            </Show>

            <Show when=move || get_result.get().is_some()>
                {move || {
                    let r = get_result.get().unwrap();
                    view! {
                        <div class="animate-in fade-in mt-6 space-y-6">
                            <StatusBadge
                                is_valid=r.is_valid
                                error_count=r.errors.len()
                                warning_count=r.warnings.len()
                            />
                            {r.summary.as_ref().map(|s| view! { <SummaryPanel summary=s.clone() /> })}
                            {r.summary.as_ref().map(|s| view! { <RecordsTable records=s.records.clone() /> })}
                            <ErrorList errors=r.errors.clone() />
                            <WarningList warnings=r.warnings.clone() />
                        </div>
                    }
                }}
            </Show>
        </div>
    }
}

// ── DropZone ──────────────────────────────────────────────────────────────────

#[component]
fn DropZone(
    on_file: impl Fn(String, Vec<u8>) + Clone + 'static,
    file_name: Signal<String>,
) -> impl IntoView {
    let on_file_drop = on_file.clone();
    let on_file_input = on_file;

    let on_drop = move |ev: DragEvent| {
        ev.prevent_default();
        if let Some(dt) = ev.data_transfer() {
            if let Some(files) = dt.files() {
                if let Some(file) = files.get(0) {
                    let name = file.name();
                    let on_file = on_file_drop.clone();
                    read_file(file, move |bytes| on_file(name.clone(), bytes));
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
                read_file(file, move |bytes| on_file(name.clone(), bytes));
            }
        }
    };

    view! {
        <div
            class=move || {
                if file_name.get().is_empty() {
                    "relative card bg-base-100 shadow-xl border-2 border-dashed border-base-300 \
                     hover:border-primary transition-all duration-300 cursor-pointer group"
                } else {
                    "relative flex items-center gap-3 px-4 py-2 rounded-box bg-base-100 shadow \
                     border border-base-300 hover:border-primary transition-colors cursor-pointer group"
                }
            }
            on:drop=on_drop
            on:dragover=|ev: DragEvent| ev.prevent_default()
        >
            <Show
                when=move || file_name.get().is_empty()
                fallback=move || {
                    view! {
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-4 w-4 shrink-0 text-base-content/40"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                            />
                        </svg>
                        <span class="font-mono text-sm truncate flex-1">{move || file_name.get()}</span>
                        <span class="btn btn-xs btn-outline gap-1 pointer-events-none shrink-0">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="h-3 w-3"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12"
                                />
                            </svg>
                            {move_tr!("dropzone-change")}
                        </span>
                    }
                }
            >
                <div class="card-body items-center text-center py-12">
                    <div class="w-16 h-16 rounded-2xl bg-primary/10 flex items-center justify-center mb-2 group-hover:-translate-y-1 transition-transform duration-300">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-8 w-8 text-primary"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"
                            />
                        </svg>
                    </div>
                    <h2 class="card-title text-lg">{move_tr!("dropzone-title")}</h2>
                    <p class="text-base-content/50 text-sm">{move_tr!("dropzone-subtitle")}</p>
                    <div class="card-actions mt-4">
                        <div class="btn btn-primary btn-sm gap-2 pointer-events-none">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="h-4 w-4"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12"
                                />
                            </svg>
                            {move_tr!("dropzone-button")}
                        </div>
                    </div>
                </div>
            </Show>
            <input
                type="file"
                accept=".720,.txt"
                on:change=on_change
                class="absolute inset-0 w-full h-full opacity-0 cursor-pointer"
            />
        </div>
    }
}

pub(crate) fn read_file(file: web_sys::File, callback: impl FnOnce(Vec<u8>) + 'static) {
    use gloo_file::File as GlooFile;
    use gloo_file::callbacks::read_as_bytes;

    let gloo_file = GlooFile::from(file);
    let task = read_as_bytes(&gloo_file, move |result| match result {
        Ok(bytes) => callback(bytes),
        Err(e) => leptos::logging::error!("Error leyendo fichero: {:?}", e),
    });
    std::mem::forget(task);
}

// ── StatusBadge ───────────────────────────────────────────────────────────────

#[component]
pub fn StatusBadge(is_valid: bool, error_count: usize, warning_count: usize) -> impl IntoView {
    if is_valid {
        let detail = if warning_count > 0 {
            tr!("status-valid-warnings", { "count" => warning_count as i64 })
        } else {
            tr!("status-valid-clean")
        };
        view! {
            <div class="alert alert-success shadow-lg">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="h-6 w-6 shrink-0"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                    />
                </svg>
                <div>
                    <h3 class="font-bold">{move_tr!("status-valid-title")}</h3>
                    <div class="text-xs opacity-80">{detail}</div>
                </div>
            </div>
        }
        .into_any()
    } else {
        let detail = tr!("status-invalid-detail", { "count" => error_count as i64 });
        view! {
            <div class="alert alert-error shadow-lg">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="h-6 w-6 shrink-0"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
                    />
                </svg>
                <div>
                    <h3 class="font-bold">{move_tr!("status-invalid-title")}</h3>
                    <div class="text-xs opacity-80">{detail}</div>
                </div>
            </div>
        }
        .into_any()
    }
}

// ── SummaryPanel ──────────────────────────────────────────────────────────────

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
    let t2_badge_class = if t2_match { "badge badge-success badge-sm" } else { "badge badge-error badge-sm" };

    view! {
        <div class="card bg-base-100 shadow-xl">
            <div class="card-body">
                <h2 class="card-title text-sm uppercase tracking-wider text-base-content/50 font-semibold mb-4">
                    {move_tr!("summary-title")}
                </h2>
                <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                    <div class="stat bg-base-200 rounded-box p-4">
                        <div class="stat-title text-xs">{move_tr!("summary-ejercicio")}</div>
                        <div class="stat-value text-2xl">{ejercicio}</div>
                    </div>
                    <div class="stat bg-base-200 rounded-box p-4">
                        <div class="stat-title text-xs">{move_tr!("summary-nif")}</div>
                        <div class="stat-value text-xl font-mono">{nif}</div>
                    </div>
                    <div class="sm:col-span-2 stat bg-base-200 rounded-box p-4">
                        <div class="stat-title text-xs">{move_tr!("summary-nombre")}</div>
                        <div class="stat-value text-lg">{nombre}</div>
                    </div>
                </div>
                <div class="divider my-2"></div>
                <div class="grid grid-cols-2 sm:grid-cols-4 gap-4">
                    <div class="text-center">
                        <div class="text-xs text-base-content/50 mb-1">{move_tr!("summary-t2-declared")}</div>
                        <div class="text-xl font-bold font-mono">{t2_decl}</div>
                    </div>
                    <div class="text-center">
                        <div class="text-xs text-base-content/50 mb-1">{move_tr!("summary-t2-real")}</div>
                        <div class="flex items-center justify-center gap-2">
                            <span class="text-xl font-bold font-mono">{t2_real}</span>
                            <span class=t2_badge_class>{if t2_match { "OK" } else { "!=" }}</span>
                        </div>
                    </div>
                    <div class="text-center">
                        <div class="text-xs text-base-content/50 mb-1">{move_tr!("summary-val1")}</div>
                        <div class="text-lg font-semibold font-mono">{val1}</div>
                    </div>
                    <div class="text-center">
                        <div class="text-xs text-base-content/50 mb-1">{move_tr!("summary-val2")}</div>
                        <div class="text-lg font-semibold font-mono">{val2}</div>
                    </div>
                </div>
            </div>
        </div>
    }
}

pub(crate) fn format_currency(val: f64) -> String {
    format_eur(val)
}

// ── RecordsTable ──────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
enum SortCol {
    Line,
    Bien,
    Pais,
    Fecha,
    Origen,
    Val1,
    Val2,
}

fn clave_label(c: char) -> (&'static str, &'static str) {
    match c {
        'C' => ("C", "badge-info"),
        'V' => ("V", "badge-secondary"),
        'I' => ("I", "badge-accent"),
        'S' => ("S", "badge-warning"),
        'B' => ("B", "badge-success"),
        _ => ("?", "badge-ghost"),
    }
}

fn format_fecha(s: &str) -> String {
    if s.len() == 8 && s != "00000000" {
        format!("{}-{}-{}", &s[0..4], &s[4..6], &s[6..8])
    } else {
        "—".to_string()
    }
}

#[component]
fn RecordsTable(records: Vec<T2Detail>) -> impl IntoView {
    if records.is_empty() {
        return view! { <div></div> }.into_any();
    }

    let count_str = format!("{}", records.len());
    let sort_col = RwSignal::new(SortCol::Line);
    let sort_asc = RwSignal::new(true);
    let stored = StoredValue::new(records);

    let sorted = Memo::new(move |_| {
        let mut v = stored.get_value();
        let col = sort_col.get();
        let asc = sort_asc.get();
        v.sort_by(|a, b| {
            let ord = match col {
                SortCol::Line => a.line.cmp(&b.line),
                SortCol::Bien => a.clave_bien.cmp(&b.clave_bien).then(a.subclave.cmp(&b.subclave)),
                SortCol::Pais => a.codigo_pais.cmp(&b.codigo_pais),
                SortCol::Fecha => a.fecha_incorporacion.cmp(&b.fecha_incorporacion),
                SortCol::Origen => a.origen.cmp(&b.origen),
                SortCol::Val1 => a.valoracion1.partial_cmp(&b.valoracion1).unwrap_or(std::cmp::Ordering::Equal),
                SortCol::Val2 => a.valoracion2.partial_cmp(&b.valoracion2).unwrap_or(std::cmp::Ordering::Equal),
            };
            if asc { ord } else { ord.reverse() }
        });
        v
    });

    let toggle = move |col: SortCol| {
        move |_| {
            if sort_col.get_untracked() == col {
                sort_asc.update(|a| *a = !*a);
            } else {
                sort_col.set(col);
                sort_asc.set(true);
            }
        }
    };

    let th_cls = move |col: SortCol, extra: &'static str| {
        move || {
            format!(
                "cursor-pointer select-none whitespace-nowrap {} {}",
                if sort_col.get() == col { "text-primary" } else { "opacity-60 hover:opacity-100" },
                extra
            )
        }
    };

    let icon = move |col: SortCol| {
        move || {
            if sort_col.get() != col { " ↕" } else if sort_asc.get() { " ↑" } else { " ↓" }
        }
    };

    view! {
        <div class="card bg-base-100 shadow-xl">
            <div class="card-body p-0">
                <div class="flex items-center gap-3 px-6 pt-5 pb-3">
                    <h3 class="font-semibold">{move_tr!("records-title")}</h3>
                    <span class="badge badge-ghost badge-sm">{count_str}</span>
                </div>
                <div class="overflow-x-auto">
                    <table class="table table-sm">
                        <thead>
                            <tr class="text-xs uppercase tracking-wider">
                                <th class=th_cls(SortCol::Line, "") on:click=toggle(SortCol::Line)>
                                    {move_tr!("records-col-line")}{icon(SortCol::Line)}
                                </th>
                                <th class=th_cls(SortCol::Bien, "") on:click=toggle(SortCol::Bien)>
                                    {move_tr!("records-col-bien")}{icon(SortCol::Bien)}
                                </th>
                                <th class=th_cls(SortCol::Pais, "") on:click=toggle(SortCol::Pais)>
                                    {move_tr!("records-col-pais")}{icon(SortCol::Pais)}
                                </th>
                                <th class=th_cls(SortCol::Fecha, "") on:click=toggle(SortCol::Fecha)>
                                    {move_tr!("records-col-fecha")}{icon(SortCol::Fecha)}
                                </th>
                                <th class=th_cls(SortCol::Origen, "") on:click=toggle(SortCol::Origen)>
                                    {move_tr!("records-col-origen")}{icon(SortCol::Origen)}
                                </th>
                                <th class=th_cls(SortCol::Val1, "text-right") on:click=toggle(SortCol::Val1)>
                                    {move_tr!("records-col-val1")}{icon(SortCol::Val1)}
                                </th>
                                <th class=th_cls(SortCol::Val2, "text-right") on:click=toggle(SortCol::Val2)>
                                    {move_tr!("records-col-val2")}{icon(SortCol::Val2)}
                                </th>
                            </tr>
                        </thead>
                        <tbody>
                            {move || {
                                sorted.get().into_iter().map(|r| {
                                    let (clave_code, clave_class) = clave_label(r.clave_bien);
                                    let badge_class = format!("badge badge-sm font-mono {}", clave_class);
                                    let clave_tip = match r.clave_bien {
                                        'C' => tr!("bien-c"),
                                        'V' => tr!("bien-v"),
                                        'I' => tr!("bien-i"),
                                        'S' => tr!("bien-s"),
                                        'B' => tr!("bien-b"),
                                        _ => tr!("bien-unknown"),
                                    };
                                    let (origen_code, origen_tip) = match r.origen {
                                        'A' => ("A", tr!("origen-a")),
                                        'M' => ("M", tr!("origen-m")),
                                        'C' => ("C", tr!("origen-c")),
                                        _ => ("?", tr!("origen-unknown")),
                                    };
                                    let val1 = format_currency(r.valoracion1);
                                    let val2 = format_currency(r.valoracion2);
                                    let fecha = format_fecha(&r.fecha_incorporacion);
                                    view! {
                                        <tr class="hover:bg-base-200 transition-colors">
                                            <td class="font-mono text-xs text-base-content/40">{r.line}</td>
                                            <td>
                                                <span class=badge_class title=clave_tip>
                                                    {format!("{}{}", clave_code, r.subclave)}
                                                </span>
                                            </td>
                                            <td class="font-mono text-xs">{r.codigo_pais}</td>
                                            <td class="font-mono text-xs">{fecha}</td>
                                            <td class="font-mono text-xs" title=origen_tip>{origen_code}</td>
                                            <td class="font-mono text-xs text-right">{val1}</td>
                                            <td class="font-mono text-xs text-right">{val2}</td>
                                        </tr>
                                    }
                                }).collect::<Vec<_>>()
                            }}
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    }
    .into_any()
}

// ── ErrorList ─────────────────────────────────────────────────────────────────

#[component]
fn ErrorList(errors: Vec<ValidationError>) -> impl IntoView {
    if errors.is_empty() {
        return view! { <div></div> }.into_any();
    }
    let count_str = format!("{}", errors.len());
    let rows: Vec<_> = errors
        .into_iter()
        .map(|e| {
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
            let row_class = format!("hover:bg-base-200 transition-colors {}", severity_class);
            view! {
                <tr class=row_class>
                    <td><span class=badge_class>{e.code}</span></td>
                    <td class="font-mono text-sm">{format!("{}", e.line)}</td>
                    <td class="font-mono text-xs text-base-content/50">{e.field}</td>
                    <td class="text-sm">{e.message}</td>
                </tr>
            }
        })
        .collect();

    view! {
        <div class="card bg-base-100 shadow-xl">
            <div class="card-body p-0">
                <div class="flex items-center gap-3 px-6 pt-5 pb-3">
                    <h3 class="font-semibold">{move_tr!("errors-title")}</h3>
                    <span class="badge badge-error badge-sm">{count_str}</span>
                </div>
                <div class="overflow-x-auto">
                    <table class="table table-sm">
                        <thead>
                            <tr class="text-xs uppercase tracking-wider">
                                <th>{move_tr!("list-col-code")}</th>
                                <th>{move_tr!("list-col-line")}</th>
                                <th>{move_tr!("list-col-field")}</th>
                                <th>{move_tr!("list-col-description")}</th>
                            </tr>
                        </thead>
                        <tbody>{rows}</tbody>
                    </table>
                </div>
            </div>
        </div>
    }
    .into_any()
}

// ── WarningList ───────────────────────────────────────────────────────────────

#[component]
fn WarningList(warnings: Vec<ValidationError>) -> impl IntoView {
    if warnings.is_empty() {
        return view! { <div></div> }.into_any();
    }
    let count_str = format!("{}", warnings.len());
    let rows: Vec<_> = warnings
        .into_iter()
        .map(|e| {
            view! {
                <tr class="hover:bg-base-200 transition-colors bg-warning/5">
                    <td><span class="badge badge-warning badge-sm font-mono">{e.code}</span></td>
                    <td class="font-mono text-sm">{format!("{}", e.line)}</td>
                    <td class="font-mono text-xs text-base-content/50">{e.field}</td>
                    <td class="text-sm">{e.message}</td>
                </tr>
            }
        })
        .collect();

    view! {
        <div class="collapse collapse-arrow bg-base-100 shadow-xl">
            <input type="checkbox" />
            <div class="collapse-title font-semibold flex items-center gap-3">
                {move_tr!("warnings-title")}
                <span class="badge badge-warning badge-sm">{count_str}</span>
            </div>
            <div class="collapse-content p-0">
                <div class="overflow-x-auto">
                    <table class="table table-sm">
                        <thead>
                            <tr class="text-xs uppercase tracking-wider">
                                <th>{move_tr!("list-col-code")}</th>
                                <th>{move_tr!("list-col-line")}</th>
                                <th>{move_tr!("list-col-field")}</th>
                                <th>{move_tr!("list-col-description")}</th>
                            </tr>
                        </thead>
                        <tbody>{rows}</tbody>
                    </table>
                </div>
            </div>
        </div>
    }
    .into_any()
}
