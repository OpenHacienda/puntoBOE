use generator720::{
    Tipo1Fields, Tipo2Fields, generate_file, import_from_bytes,
    BIEN_CUENTA, BIEN_VALORES, BIEN_IIC, BIEN_SEGURO, BIEN_INMUEBLE,
    ORIGEN_ALTA, ORIGEN_MODIFICACION, ORIGEN_CANCELACION,
    ID_VALORES_NINGUNA, ID_VALORES_ISIN, ID_VALORES_OTRO,
    CUENTA_IBAN, CUENTA_OTRO,
    REPR_NOMINATIVOS, REPR_AL_PORTADOR,
    INMUEBLE_URBANO, INMUEBLE_RUSTICO,
};
use leptos::prelude::*;
use leptos::*;
use leptos_fluent::move_tr;
use validator720::validate;
use wasm_bindgen::JsCast;
use std::collections::HashSet;
use web_sys::{DragEvent, Event, HtmlInputElement, HtmlSelectElement};

use crate::{StatusBadge, format_currency, read_file, trigger_download};

// ── Helpers ───────────────────────────────────────────────────────────────────

/// YYYYMMDD  →  YYYY-MM-DD  (for <input type="date">)
fn date_to_html(d: &str) -> String {
    if d.len() == 8 && d != "00000000" && d.chars().all(|c| c.is_ascii_digit()) {
        format!("{}-{}-{}", &d[0..4], &d[4..6], &d[6..8])
    } else {
        String::new()
    }
}

/// YYYY-MM-DD  →  YYYYMMDD
fn html_to_date(d: &str) -> String {
    let digits: String = d.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.len() == 8 { digits } else { "00000000".to_string() }
}

fn input_val(ev: &Event) -> String {
    ev.target()
        .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
        .map(|i| i.value())
        .unwrap_or_default()
}

fn select_val(ev: &Event) -> String {
    ev.target()
        .and_then(|t| t.dyn_into::<HtmlSelectElement>().ok())
        .map(|s| s.value())
        .unwrap_or_default()
}

// ── Entry point ───────────────────────────────────────────────────────────────

#[component]
pub fn EditorPage() -> impl IntoView {
    // Central state
    let t1 = RwSignal::new(Tipo1Fields::default());
    // Records stored as (stable_id, fields) to preserve row identity across adds/removes
    let records: RwSignal<Vec<(u64, Tipo2Fields)>> = RwSignal::new(Vec::new());
    let next_id: RwSignal<u64> = RwSignal::new(0);

    // Drag-and-drop state
    let drag_src: RwSignal<Option<u64>> = RwSignal::new(None);
    let drag_over: RwSignal<Option<u64>> = RwSignal::new(None);

    // Collapse state — set of row ids that are currently collapsed
    let collapsed_ids: RwSignal<HashSet<u64>> = RwSignal::new(HashSet::new());
    let all_collapsed = Memo::new(move |_| {
        let recs = records.get();
        !recs.is_empty() && recs.iter().all(|(id, _)| collapsed_ids.get().contains(id))
    });

    // Import handler
    let on_import = move |_name: String, bytes: Vec<u8>| {
        match import_from_bytes(&bytes) {
            Ok((new_t1, new_t2s)) => {
                t1.set(new_t1);
                let mut id = next_id.get_untracked();
                let entries: Vec<(u64, Tipo2Fields)> = new_t2s
                    .into_iter()
                    .map(|f| {
                        let entry = (id, f);
                        id += 1;
                        entry
                    })
                    .collect();
                next_id.set(id);
                records.set(entries);
            }
            Err(e) => leptos::logging::error!("Import error: {}", e),
        }
    };

    // Add a blank Tipo2 record
    let add_record = move |_| {
        let id = next_id.get_untracked();
        next_id.update(|n| *n += 1);
        // Pre-fill ejercicio-coherent defaults
        let ejercicio = t1.get_untracked().ejercicio.clone();
        let nif_decl = t1.get_untracked().nif_declarante.clone();
        records.update(|v| {
            v.push((id, Tipo2Fields {
                fecha_incorporacion: format!("{}0101", ejercicio),
                ..Default::default()
            }));
        });
        let _ = ejercicio;
        let _ = nif_decl;
    };

    // Download handler
    let on_download = move |_| {
        let fields: Vec<Tipo2Fields> = records.get_untracked().into_iter().map(|(_, f)| f).collect();
        let bytes = generate_file(&t1.get_untracked(), &fields);
        trigger_download(&bytes, "declaracion.720");
    };

    // Live validation — recomputes whenever t1 or records change.
    // Signal::derive is used because ValidationResult does not implement PartialEq.
    let validation = Signal::derive(move || {
        let fields: Vec<Tipo2Fields> = records.get().into_iter().map(|(_, f)| f).collect();
        let bytes = generate_file(&t1.get(), &fields);
        validate(&bytes)
    });

    view! {
        <div class="space-y-6">
            // ── Import bar ────────────────────────────────────────────────────
            <ImportBar on_file=on_import />

            // ── Tipo 1 form ───────────────────────────────────────────────────
            <Tipo1Form t1=t1 />

            // ── Tipo 2 records ────────────────────────────────────────────────
            <div class="card bg-base-100 shadow-xl">
                <div class="card-body">
                    <div class="flex items-center justify-between mb-4">
                        <h2 class="card-title text-sm uppercase tracking-wider text-base-content/50 font-semibold">
                            {move_tr!("editor-t2-section")}
                            <span class="badge badge-ghost badge-sm ml-2">
                                {move || records.get().len().to_string()}
                            </span>
                        </h2>
                        <div class="flex items-center gap-2">
                        <button
                            class="btn btn-ghost btn-sm gap-1.5"
                            title=move || if all_collapsed.get() { "Expandir todo" } else { "Colapsar todo" }
                            on:click=move |_| {
                                if all_collapsed.get() {
                                    collapsed_ids.update(|s| s.clear());
                                } else {
                                    let ids: HashSet<u64> = records.get_untracked().iter().map(|(id, _)| *id).collect();
                                    collapsed_ids.set(ids);
                                }
                            }
                        >
                            {move || if all_collapsed.get() {
                                view! {
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 8h16M4 16h16" />
                                    </svg>
                                    <span class="text-xs">{"Expandir todo"}</span>
                                }.into_any()
                            } else {
                                view! {
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 15l7-7 7 7" />
                                    </svg>
                                    <span class="text-xs">{"Colapsar todo"}</span>
                                }.into_any()
                            }}
                        </button>
                        <button class="btn btn-primary btn-sm gap-2" on:click=add_record>
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                            </svg>
                            {move_tr!("editor-add-record")}
                        </button>
                        </div>
                    </div>
                    <div class="space-y-4">
                        <For
                            each={move || records.get().iter().map(|(id, _)| *id).collect::<Vec<u64>>()}
                            key={|id| *id}
                            let(id)
                        >
                            <Tipo2RecordRow id=id records=records drag_src=drag_src drag_over=drag_over collapsed_ids=collapsed_ids />
                        </For>
                    </div>
                    <Show when=move || records.get().is_empty()>
                        <div class="text-center py-8 text-base-content/30 text-sm">
                            {move_tr!("editor-no-records")}
                        </div>
                    </Show>
                    <Show when=move || !records.get().is_empty()>
                        <button class="btn btn-primary btn-sm btn-outline w-full gap-2 mt-2" on:click=add_record>
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                            </svg>
                            {move_tr!("editor-add-record")}
                        </button>
                    </Show>
                </div>
            </div>

            // ── Live validation + download ─────────────────────────────────────
            <div class="card bg-base-100 shadow-xl">
                <div class="card-body">
                    <h2 class="card-title text-sm uppercase tracking-wider text-base-content/50 font-semibold mb-4">
                        {move_tr!("editor-preview")}
                    </h2>
                    {move || {
                        let r = validation.get();
                        view! {
                            <StatusBadge
                                is_valid=r.is_valid
                                error_count=r.errors.len()
                                warning_count=r.warnings.len()
                            />
                        }
                    }}
                    <div class="card-actions justify-end mt-4">
                        <button
                            class="btn btn-primary gap-2"
                            on:click=on_download
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                    d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
                            </svg>
                            {move_tr!("editor-download")}
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}

// ── ImportBar ─────────────────────────────────────────────────────────────────

#[component]
fn ImportBar(on_file: impl Fn(String, Vec<u8>) + Clone + 'static) -> impl IntoView {
    let on_change = move |ev: Event| {
        let target: HtmlInputElement = ev.target().unwrap().unchecked_into();
        if let Some(files) = target.files() {
            if let Some(file) = files.get(0) {
                let name = file.name();
                let on_file = on_file.clone();
                read_file(file, move |bytes| on_file(name.clone(), bytes));
            }
        }
    };

    view! {
        <div class="flex items-center gap-3 px-4 py-3 rounded-box bg-base-100 shadow border border-base-300">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-base-content/40 shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12" />
            </svg>
            <span class="text-sm text-base-content/60 flex-1">{move_tr!("editor-import-hint")}</span>
            <label class="btn btn-outline btn-sm gap-2 cursor-pointer">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M9 11l3 3m0 0l3-3m-3 3V4" />
                </svg>
                {move_tr!("editor-import")}
                <input
                    type="file"
                    accept=".720,.txt"
                    class="hidden"
                    on:change=on_change
                />
            </label>
        </div>
    }
}

// ── Tipo 1 Form ───────────────────────────────────────────────────────────────

#[component]
fn Tipo1Form(t1: RwSignal<Tipo1Fields>) -> impl IntoView {
    view! {
        <div class="card bg-base-100 shadow-xl">
            <div class="card-body">
                <h2 class="card-title text-sm uppercase tracking-wider text-base-content/50 font-semibold mb-4">
                    {move_tr!("editor-t1-section")}
                </h2>
                <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">

                    // Ejercicio
                    <div class="form-control">
                        <label class="label pb-1">
                            <span class="label-text text-xs font-semibold">{move_tr!("editor-field-ejercicio")}</span>
                        </label>
                        <input
                            type="text"
                            maxlength="4"
                            placeholder="2024"
                            class="input input-bordered input-sm font-mono"
                            prop:value=move || t1.get().ejercicio.clone()
                            on:input=move |ev| {
                                let v = input_val(&ev);
                                t1.update(|f| f.ejercicio = v);
                            }
                        />
                    </div>

                    // NIF declarante
                    <div class="form-control">
                        <label class="label pb-1">
                            <span class="label-text text-xs font-semibold">{move_tr!("editor-field-nif-declarante")}</span>
                        </label>
                        <input
                            type="text"
                            maxlength="9"
                            placeholder="12345678Z"
                            class="input input-bordered input-sm font-mono uppercase"
                            prop:value=move || t1.get().nif_declarante.clone()
                            on:input=move |ev| {
                                let v = input_val(&ev).to_uppercase();
                                t1.update(|f| f.nif_declarante = v);
                            }
                        />
                    </div>

                    // Nombre declarante
                    <div class="form-control sm:col-span-2">
                        <label class="label pb-1">
                            <span class="label-text text-xs font-semibold">{move_tr!("editor-field-nombre-declarante")}</span>
                        </label>
                        <input
                            type="text"
                            maxlength="40"
                            placeholder="APELLIDOS NOMBRE o RAZÓN SOCIAL"
                            class="input input-bordered input-sm uppercase"
                            prop:value=move || t1.get().nombre_declarante.clone()
                            on:input=move |ev| {
                                let v = input_val(&ev).to_uppercase();
                                t1.update(|f| f.nombre_declarante = v);
                            }
                        />
                    </div>

                    // NIF presentador
                    <div class="form-control">
                        <label class="label pb-1">
                            <span class="label-text text-xs font-semibold">{move_tr!("editor-field-nif-presentador")}</span>
                            <span class="label-text-alt opacity-50">{move_tr!("editor-field-optional")}</span>
                        </label>
                        <input
                            type="text"
                            maxlength="9"
                            placeholder="Igual al declarante si se omite"
                            class="input input-bordered input-sm font-mono uppercase"
                            prop:value=move || t1.get().nif_presentador.clone()
                            on:input=move |ev| {
                                let v = input_val(&ev).to_uppercase();
                                t1.update(|f| f.nif_presentador = v);
                            }
                        />
                    </div>

                    // Nombre presentador
                    <div class="form-control">
                        <label class="label pb-1">
                            <span class="label-text text-xs font-semibold">{move_tr!("editor-field-nombre-presentador")}</span>
                            <span class="label-text-alt opacity-50">{move_tr!("editor-field-optional")}</span>
                        </label>
                        <input
                            type="text"
                            maxlength="40"
                            placeholder="Igual al declarante si se omite"
                            class="input input-bordered input-sm uppercase"
                            prop:value=move || t1.get().nombre_presentador.clone()
                            on:input=move |ev| {
                                let v = input_val(&ev).to_uppercase();
                                t1.update(|f| f.nombre_presentador = v);
                            }
                        />
                    </div>

                    // Num identificativo
                    <div class="form-control">
                        <label class="label pb-1">
                            <span class="label-text text-xs font-semibold">{move_tr!("editor-field-num-identificativo")}</span>
                        </label>
                        <input
                            type="text"
                            maxlength="13"
                            placeholder="7200000000001"
                            class="input input-bordered input-sm font-mono"
                            prop:value=move || t1.get().num_identificativo.clone()
                            on:input=move |ev| {
                                let v = input_val(&ev);
                                t1.update(|f| f.num_identificativo = v);
                            }
                        />
                        <div class="label pt-0.5">
                            <span class="label-text-alt opacity-40">"Generado automáticamente al descargar si se deja vacío"</span>
                        </div>
                    </div>

                    // Tipo declaración
                    <div class="form-control">
                        <label class="label pb-1">
                            <span class="label-text text-xs font-semibold">{move_tr!("editor-field-tipo-dec")}</span>
                        </label>
                        <select
                            class="select select-bordered select-sm"
                            prop:value=move || {
                                if t1.get().dec_sustitutiva { "S" }
                                else if t1.get().dec_complementaria { "C" }
                                else { "N" }
                            }
                            on:change=move |ev| {
                                let v = select_val(&ev);
                                t1.update(|f| {
                                    f.dec_complementaria = v == "C";
                                    f.dec_sustitutiva = v == "S";
                                });
                            }
                        >
                            <option value="N">{move_tr!("editor-dec-normal")}</option>
                            <option value="C">{move_tr!("editor-dec-complementaria")}</option>
                            <option value="S">{move_tr!("editor-dec-sustitutiva")}</option>
                        </select>
                    </div>

                    // Num declaración anterior (only when sustitutiva)
                    {move || t1.get().dec_sustitutiva.then(|| view! {
                        <div class="form-control sm:col-span-2">
                            <label class="label pb-1">
                                <span class="label-text text-xs font-semibold">{move_tr!("editor-field-num-dec-anterior")}</span>
                            </label>
                            <input
                                type="text"
                                maxlength="13"
                                placeholder="0000000000000"
                                class="input input-bordered input-sm font-mono"
                                prop:value=move || t1.get().num_dec_anterior.clone()
                                on:input=move |ev| {
                                    let v = input_val(&ev);
                                    t1.update(|f| f.num_dec_anterior = v);
                                }
                            />
                        </div>
                    })}
                </div>
            </div>
        </div>
    }
}

// ── Tipo 2 Record Row ─────────────────────────────────────────────────────────

#[component]
fn Tipo2RecordRow(
    id: u64,
    records: RwSignal<Vec<(u64, Tipo2Fields)>>,
    drag_src: RwSignal<Option<u64>>,
    drag_over: RwSignal<Option<u64>>,
    collapsed_ids: RwSignal<HashSet<u64>>,
) -> impl IntoView {
    // Reactive snapshot of this row’s fields — re-derives whenever records changes
    // without remounting the component, keeping focus alive while typing.
    let row = Memo::new(move |_| {
        records.with(|v| {
            v.iter()
                .find(|(i, _)| *i == id)
                .map(|(_, f)| f.clone())
                .unwrap_or_default()
        })
    });

    let show_body = move || !collapsed_ids.get().contains(&id);

    // Helper: update a single field in this row by id
    macro_rules! upd {
        ($field:ident, $val:expr) => {{
            let value = $val;
            records.update(|v| {
                if let Some((_, r)) = v.iter_mut().find(|(i, _)| *i == id) {
                    r.$field = value;
                }
            });
        }};
    }

    let badge_class = move || match row.with(|r| r.clave_tipo_bien) {
        BIEN_CUENTA   => "badge badge-info badge-sm font-mono",
        BIEN_VALORES  => "badge badge-secondary badge-sm font-mono",
        BIEN_IIC      => "badge badge-accent badge-sm font-mono",
        BIEN_SEGURO   => "badge badge-warning badge-sm font-mono",
        BIEN_INMUEBLE => "badge badge-success badge-sm font-mono",
        _             => "badge badge-ghost badge-sm font-mono",
    };

    view! {
        <div
            class=move || {
                let is_over = drag_over.get() == Some(id);
                let is_src  = drag_src.get()  == Some(id);
                format!(
                    "border rounded-box overflow-hidden transition-all {}",
                    if is_src       { "border-base-300 opacity-50" }
                    else if is_over { "border-primary border-2" }
                    else            { "border-base-300" }
                )
            }
            on:dragover=move |ev: DragEvent| {
                ev.prevent_default();
                if drag_src.get_untracked().is_some() {
                    drag_over.set(Some(id));
                }
            }
            on:drop=move |ev: DragEvent| {
                ev.prevent_default();
                if let Some(src_id) = drag_src.get_untracked() {
                    if src_id != id {
                        records.update(|v| {
                            if let Some(src_pos) = v.iter().position(|(i, _)| *i == src_id) {
                                if let Some(dst_pos) = v.iter().position(|(i, _)| *i == id) {
                                    let item = v.remove(src_pos);
                                    let insert_at = if src_pos < dst_pos { dst_pos - 1 } else { dst_pos };
                                    v.insert(insert_at, item);
                                }
                            }
                        });
                    }
                }
                drag_src.set(None);
                drag_over.set(None);
            }
        >
            // ── Row header ────────────────────────────────────────────────────
            <div class="flex items-center gap-3 px-4 py-3 bg-base-200">
                // Drag handle
                <div
                    draggable="true"
                    on:dragstart=move |ev: DragEvent| {
                        drag_src.set(Some(id));
                        if let Some(dt) = ev.data_transfer() {
                            let _ = dt.set_data("text/plain", &id.to_string());
                            dt.set_effect_allowed("move");
                        }
                    }
                    on:dragend=move |_| {
                        drag_src.set(None);
                        drag_over.set(None);
                    }
                    class="cursor-grab active:cursor-grabbing opacity-30 hover:opacity-70 shrink-0 px-0.5"
                    title="Arrastrar para reordenar"
                >
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
                        <circle cx="7" cy="4"  r="1.5"/><circle cx="13" cy="4"  r="1.5"/>
                        <circle cx="7" cy="10" r="1.5"/><circle cx="13" cy="10" r="1.5"/>
                        <circle cx="7" cy="16" r="1.5"/><circle cx="13" cy="16" r="1.5"/>
                    </svg>
                </div>
                // Collapse chevron
                <button
                    class="btn btn-ghost btn-xs opacity-40 hover:opacity-80 shrink-0"
                    title=move || if show_body() { "Colapsar" } else { "Expandir" }
                    on:click=move |_| collapsed_ids.update(|s| {
                        if s.contains(&id) { s.remove(&id); } else { s.insert(id); }
                    })
                >
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 transition-transform" style=move || if show_body() { "" } else { "transform:rotate(-90deg)" } fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
                    </svg>
                </button>
                <span class=badge_class>{move || row.with(|r| format!("{}{}", r.clave_tipo_bien, r.subclave))}</span>
                <span class="text-sm font-medium flex-1 truncate">{move || row.with(|r|
                    if r.nombre_declarado.is_empty() { format!("Registro {}", id + 1) }
                    else { r.nombre_declarado.clone() }
                )}</span>
                <span class="text-xs font-mono text-base-content/50">{move || format_currency(row.with(|r| r.valoracion1))}</span>
                <button
                    class="btn btn-ghost btn-xs opacity-50 hover:opacity-100"
                    title="Mover arriba"
                    on:click=move |_| {
                        records.update(|v| {
                            if let Some(pos) = v.iter().position(|(i, _)| *i == id) {
                                if pos > 0 { v.swap(pos, pos - 1); }
                            }
                        });
                    }
                >
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 15l7-7 7 7" />
                    </svg>
                </button>
                <button
                    class="btn btn-ghost btn-xs opacity-50 hover:opacity-100"
                    title="Mover abajo"
                    on:click=move |_| {
                        records.update(|v| {
                            if let Some(pos) = v.iter().position(|(i, _)| *i == id) {
                                if pos + 1 < v.len() { v.swap(pos, pos + 1); }
                            }
                        });
                    }
                >
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
                    </svg>
                </button>
                <button
                    class="btn btn-ghost btn-xs text-error"
                    title="Eliminar registro"
                    on:click=move |_| {
                        records.update(|v| v.retain(|(i, _)| *i != id));
                    }
                >
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                    </svg>
                </button>
            </div>

            // ── Fields grid ───────────────────────────────────────────────────
            <div class="p-4 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3"
                style=move || if show_body() { "" } else { "display:none" }
            >

                // ── Sección: Declarado ─────────────────────────────────────
                <div class="sm:col-span-2 lg:col-span-3">
                    <div class="divider text-xs opacity-40 my-1">{move_tr!("editor-section-declarado")}</div>
                </div>

                // NIF declarado
                <div class="form-control">
                    <label class="label pb-1">
                        <span class="label-text text-xs">{move_tr!("editor-field-nif-declarado")}</span>
                    </label>
                    <input
                        type="text" maxlength="9"
                        class="input input-bordered input-xs font-mono uppercase"
                        prop:value=move || row.with(|r| r.nif_declarado.clone())
                        on:input=move |ev| upd!(nif_declarado, input_val(&ev).to_uppercase())
                    />
                </div>

                // Nombre declarado
                <div class="form-control sm:col-span-1 lg:col-span-2">
                    <label class="label pb-1">
                        <span class="label-text text-xs">{move_tr!("editor-field-nombre-declarado")}</span>
                    </label>
                    <input
                        type="text" maxlength="40"
                        class="input input-bordered input-xs uppercase"
                        prop:value=move || row.with(|r| r.nombre_declarado.clone())
                        on:input=move |ev| upd!(nombre_declarado, input_val(&ev).to_uppercase())
                    />
                </div>

                // Clave condición
                <div class="form-control">
                    <label class="label pb-1">
                        <span class="label-text text-xs">{move_tr!("editor-field-clave-condicion")}</span>
                    </label>
                    <select
                        class="select select-bordered select-xs"
                        prop:value=move || row.with(|r| r.clave_condicion.to_string())
                        on:change=move |ev| {
                            let c = select_val(&ev).chars().next().unwrap_or('1');
                            upd!(clave_condicion, c);
                        }
                    >
                        <option value="1">"Titular"</option>
                        <option value="2">"Representante legal"</option>
                        <option value="3">"Autorizado"</option>
                        <option value="4">"Beneficiario"</option>
                        <option value="5">"Tomador de seguro"</option>
                        <option value="6">"Con poderes"</option>
                        <option value="7">"Cesionario"</option>
                        <option value="8">"Titular real"</option>
                    </select>
                </div>

                // ── Sección: Tipo de bien ──────────────────────────────────
                <div class="sm:col-span-2 lg:col-span-3">
                    <div class="divider text-xs opacity-40 my-1">{move_tr!("editor-section-tipo-bien")}</div>
                </div>

                // Clave tipo bien
                <div class="form-control">
                    <label class="label pb-1">
                        <span class="label-text text-xs">{move_tr!("editor-field-clave-bien")}</span>
                    </label>
                    <select
                        class="select select-bordered select-xs"
                        prop:value=move || row.with(|r| r.clave_tipo_bien.to_string())
                        on:change=move |ev| {
                            let c = select_val(&ev).chars().next().unwrap_or(BIEN_CUENTA);
                            records.update(|v| {
                                if let Some((_, r)) = v.iter_mut().find(|(i, _)| *i == id) {
                                    r.clave_tipo_bien = c;
                                    // reset subclave to a sensible default
                                    r.subclave = match c {
                                        BIEN_CUENTA | BIEN_VALORES | BIEN_SEGURO | BIEN_INMUEBLE => '1',
                                        BIEN_IIC => '0',
                                        _ => '1',
                                    };
                                    // reset clave_id_cuenta when not C
                                    if c != BIEN_CUENTA { r.clave_id_cuenta = ' '; }
                                    else { r.clave_id_cuenta = CUENTA_IBAN; }
                                    // reset clave_represent_valores
                                    r.clave_represent_valores = if matches!(c, BIEN_VALORES | BIEN_IIC) { REPR_NOMINATIVOS } else { ' ' };
                                    // reset clave_tipo_inmueble
                                    r.clave_tipo_inmueble = if c == BIEN_INMUEBLE { INMUEBLE_URBANO } else { ' ' };
                                    // reset identification fields when switching away from V/I
                                    if !matches!(c, BIEN_VALORES | BIEN_IIC) {
                                        r.clave_identificacion = ID_VALORES_NINGUNA;
                                        r.identificacion_valores = String::new();
                                    }
                                }
                            });
                        }
                    >
                        <option value={BIEN_CUENTA.to_string()}>"Cuenta bancaria"</option>
                        <option value={BIEN_VALORES.to_string()}>"Valores mobiliarios"</option>
                        <option value={BIEN_IIC.to_string()}>"Institución de Inversión Colectiva (IIC)"</option>
                        <option value={BIEN_SEGURO.to_string()}>"Seguro o renta"</option>
                        <option value={BIEN_INMUEBLE.to_string()}>"Bien inmueble"</option>
                    </select>
                </div>

                // Subclave
                <div class="form-control">
                    <label class="label pb-1">
                        <span class="label-text text-xs">{move_tr!("editor-field-subclave")}</span>
                    </label>
                    {move || {
                        let bien = row.with(|r| r.clave_tipo_bien);
                        let current_sub = row.with(|r| r.subclave);
                        let options: Vec<(char, &str)> = match bien {
                            BIEN_CUENTA => vec![
                                ('1', "Cuenta corriente"),
                                ('2', "Cuenta de ahorro"),
                                ('3', "Plazo / depósito"),
                                ('4', "Cuenta de crédito"),
                                ('5', "Otras cuentas"),
                            ],
                            BIEN_VALORES => vec![
                                ('1', "Acciones y otros valores"),
                                ('2', "Acciones en IIC"),
                                ('3', "Otros valores"),
                            ],
                            BIEN_IIC => vec![('0', "Participaciones en IIC")],
                            BIEN_SEGURO => vec![
                                ('1', "Seguro de vida"),
                                ('2', "Renta temporal o vitalicia"),
                            ],
                            BIEN_INMUEBLE => vec![
                                ('1', "Plena propiedad"),
                                ('2', "Nuda propiedad"),
                                ('3', "Derecho de usufructo"),
                                ('4', "Multipropiedad"),
                                ('5', "Otros derechos reales"),
                            ],
                            _ => vec![('1', "Otro")],
                        };
                        view! {
                            <select
                                class="select select-bordered select-xs"
                                prop:value=current_sub.to_string()
                                on:change=move |ev| {
                                    let c = select_val(&ev).chars().next().unwrap_or('1');
                                    upd!(subclave, c);
                                }
                            >
                                {options.into_iter().map(|(v, label)| view! {
                                    <option value=v.to_string()>{label}</option>
                                }).collect_view()}
                            </select>
                        }
                    }}
                </div>

                // País
                <div class="form-control">
                    <label class="label pb-1">
                        <span class="label-text text-xs">{move_tr!("editor-field-pais")}</span>
                    </label>
                    <input
                        type="text" maxlength="2"
                        placeholder="ES"
                        class="input input-bordered input-xs font-mono uppercase"
                        prop:value=move || row.with(|r| r.codigo_pais.clone())
                        on:input=move |ev| upd!(codigo_pais, input_val(&ev).to_uppercase())
                    />
                </div>

                // ── Sección: Identificación ────────────────────────────────
                <div class="sm:col-span-2 lg:col-span-3">
                    <div class="divider text-xs opacity-40 my-1">{move_tr!("editor-section-identificacion")}</div>
                </div>

                // Clave identificación valores (solo V/I)
                {move || {
                    let bien = row.with(|r| r.clave_tipo_bien);
                    matches!(bien, BIEN_VALORES | BIEN_IIC).then(|| view! {
                        <>
                            <div class="form-control">
                                <label class="label pb-1">
                                    <span class="label-text text-xs">{move_tr!("editor-field-clave-id-valores")}</span>
                                </label>
                                <select
                                    class="select select-bordered select-xs"
                                    prop:value=move || row.with(|r| r.clave_identificacion.to_string())
                                    on:change=move |ev| {
                                        let c = select_val(&ev).chars().next().unwrap_or(ID_VALORES_NINGUNA);
                                        upd!(clave_identificacion, c);
                                    }
                                >
                                    <option value={ID_VALORES_NINGUNA.to_string()}>"Sin identificación"</option>
                                    <option value={ID_VALORES_ISIN.to_string()}>"ISIN"</option>
                                    <option value={ID_VALORES_OTRO.to_string()}>"Otro código"</option>
                                </select>
                            </div>
                            <div class="form-control">
                                <label class="label pb-1">
                                    <span class="label-text text-xs">{move_tr!("editor-field-id-valores")}</span>
                                </label>
                                <input
                                    type="text" maxlength="12"
                                    placeholder="ISIN / código"
                                    class="input input-bordered input-xs font-mono uppercase"
                                    prop:value=move || row.with(|r| r.identificacion_valores.clone())
                                    on:input=move |ev| upd!(identificacion_valores, input_val(&ev).to_uppercase())
                                />
                            </div>
                        </>
                    })
                }}

                // Clave ID cuenta + IBAN (solo C)
                {move || {
                    let bien = row.with(|r| r.clave_tipo_bien);
                    (bien == BIEN_CUENTA).then(|| view! {
                        <>
                            <div class="form-control">
                                <label class="label pb-1">
                                    <span class="label-text text-xs">{move_tr!("editor-field-clave-id-cuenta")}</span>
                                </label>
                                <select
                                    class="select select-bordered select-xs"
                                    prop:value=move || row.with(|r| r.clave_id_cuenta.to_string())
                                    on:change=move |ev| {
                                        let c = select_val(&ev).chars().next().unwrap_or(CUENTA_IBAN);
                                        upd!(clave_id_cuenta, c);
                                    }
                                >
                                    <option value={CUENTA_IBAN.to_string()}>"IBAN"</option>
                                    <option value={CUENTA_OTRO.to_string()}>"Otro identificador"</option>
                                </select>
                            </div>
                            <div class="form-control">
                                <label class="label pb-1">
                                    <span class="label-text text-xs">"IBAN / Número cuenta"</span>
                                </label>
                                <input
                                    type="text" maxlength="34"
                                    placeholder="CH9300762011623852957"
                                    class="input input-bordered input-xs font-mono uppercase"
                                    prop:value=move || row.with(|r| r.identificacion_cuenta.clone())
                                    on:input=move |ev| upd!(identificacion_cuenta, input_val(&ev).to_uppercase())
                                />
                            </div>
                            <div class="form-control">
                                <label class="label pb-1">
                                    <span class="label-text text-xs">"BIC / SWIFT"</span>
                                </label>
                                <input
                                    type="text" maxlength="11"
                                    placeholder="UBSWCHZH80A"
                                    class="input input-bordered input-xs font-mono uppercase"
                                    prop:value=move || row.with(|r| r.codigo_bic.clone())
                                    on:input=move |ev| upd!(codigo_bic, input_val(&ev).to_uppercase())
                                />
                            </div>
                        </>
                    })
                }}

                // Clave represent. + Num valores (solo V/I)
                {move || {
                    let bien = row.with(|r| r.clave_tipo_bien);
                    matches!(bien, BIEN_VALORES | BIEN_IIC).then(|| view! {
                        <>
                            <div class="form-control">
                                <label class="label pb-1">
                                    <span class="label-text text-xs">{move_tr!("editor-field-clave-repr")}</span>
                                </label>
                                <select
                                    class="select select-bordered select-xs"
                                    prop:value=move || row.with(|r| r.clave_represent_valores.to_string())
                                    on:change=move |ev| {
                                        let c = select_val(&ev).chars().next().unwrap_or(REPR_NOMINATIVOS);
                                        upd!(clave_represent_valores, c);
                                    }
                                >
                                    <option value={REPR_NOMINATIVOS.to_string()}>"Nominativos"</option>
                                    <option value={REPR_AL_PORTADOR.to_string()}>"Al portador"</option>
                                </select>
                            </div>
                            <div class="form-control">
                                <label class="label pb-1">
                                    <span class="label-text text-xs">{move_tr!("editor-field-num-valores")}</span>
                                </label>
                                <input
                                    type="number" min="0"
                                    class="input input-bordered input-xs font-mono"
                                    prop:value=move || row.with(|r| r.num_valores.to_string())
                                    on:input=move |ev| {
                                        let n: u64 = input_val(&ev).parse().unwrap_or(0);
                                        upd!(num_valores, n);
                                    }
                                />
                            </div>
                        </>
                    })
                }}

                // Clave tipo inmueble (solo B)
                {move || {
                    let bien = row.with(|r| r.clave_tipo_bien);
                    (bien == BIEN_INMUEBLE).then(|| view! {
                        <div class="form-control">
                            <label class="label pb-1">
                                <span class="label-text text-xs">{move_tr!("editor-field-tipo-inmueble")}</span>
                            </label>
                            <select
                                class="select select-bordered select-xs"
                                prop:value=move || row.with(|r| r.clave_tipo_inmueble.to_string())
                                on:change=move |ev| {
                                    let c = select_val(&ev).chars().next().unwrap_or(INMUEBLE_URBANO);
                                    upd!(clave_tipo_inmueble, c);
                                }
                            >
                                <option value={INMUEBLE_URBANO.to_string()}>"Urbano"</option>
                                <option value={INMUEBLE_RUSTICO.to_string()}>"Rústico"</option>
                            </select>
                        </div>
                    })
                }}

                // ── Sección: Entidad ───────────────────────────────────────
                <div class="sm:col-span-2 lg:col-span-3">
                    <div class="divider text-xs opacity-40 my-1">{move_tr!("editor-section-entidad")}</div>
                </div>

                <div class="form-control sm:col-span-2">
                    <label class="label pb-1">
                        <span class="label-text text-xs">{move_tr!("editor-field-nombre-entidad")}</span>
                    </label>
                    <input
                        type="text" maxlength="41"
                        class="input input-bordered input-xs uppercase"
                        prop:value=move || row.with(|r| r.identificacion_entidad.clone())
                        on:input=move |ev| upd!(identificacion_entidad, input_val(&ev).to_uppercase())
                    />
                </div>

                <div class="form-control">
                    <label class="label pb-1">
                        <span class="label-text text-xs">{move_tr!("editor-field-nif-fiscal-pais")}</span>
                    </label>
                    <input
                        type="text" maxlength="20"
                        class="input input-bordered input-xs font-mono uppercase"
                        prop:value=move || row.with(|r| r.nif_fiscal_pais.clone())
                        on:input=move |ev| upd!(nif_fiscal_pais, input_val(&ev).to_uppercase())
                    />
                </div>

                <div class="form-control sm:col-span-2">
                    <label class="label pb-1">
                        <span class="label-text text-xs">{move_tr!("editor-field-domicilio")}</span>
                    </label>
                    <input
                        type="text" maxlength="40"
                        class="input input-bordered input-xs uppercase"
                        prop:value=move || row.with(|r| r.domicilio.clone())
                        on:input=move |ev| upd!(domicilio, input_val(&ev).to_uppercase())
                    />
                </div>

                <div class="form-control">
                    <label class="label pb-1">
                        <span class="label-text text-xs">{move_tr!("editor-field-localidad")}</span>
                    </label>
                    <input
                        type="text" maxlength="30"
                        class="input input-bordered input-xs uppercase"
                        prop:value=move || row.with(|r| r.localidad.clone())
                        on:input=move |ev| upd!(localidad, input_val(&ev).to_uppercase())
                    />
                </div>

                <div class="form-control">
                    <label class="label pb-1">
                        <span class="label-text text-xs">{move_tr!("editor-field-municipio")}</span>
                    </label>
                    <input
                        type="text" maxlength="30"
                        class="input input-bordered input-xs uppercase"
                        prop:value=move || row.with(|r| r.municipio.clone())
                        on:input=move |ev| upd!(municipio, input_val(&ev).to_uppercase())
                    />
                </div>

                <div class="form-control">
                    <label class="label pb-1">
                        <span class="label-text text-xs">{move_tr!("editor-field-cod-postal")}</span>
                    </label>
                    <input
                        type="text" maxlength="10"
                        class="input input-bordered input-xs font-mono"
                        prop:value=move || row.with(|r| r.codigo_postal.clone())
                        on:input=move |ev| upd!(codigo_postal, input_val(&ev))
                    />
                </div>

                <div class="form-control">
                    <label class="label pb-1">
                        <span class="label-text text-xs">{move_tr!("editor-field-pais-domicilio")}</span>
                    </label>
                    <input
                        type="text" maxlength="2"
                        placeholder="CH"
                        class="input input-bordered input-xs font-mono uppercase"
                        prop:value=move || row.with(|r| r.pais_domicilio.clone())
                        on:input=move |ev| upd!(pais_domicilio, input_val(&ev).to_uppercase())
                    />
                </div>

                // ── Sección: Valoración ────────────────────────────────────
                <div class="sm:col-span-2 lg:col-span-3">
                    <div class="divider text-xs opacity-40 my-1">{move_tr!("editor-section-valoracion")}</div>
                </div>

                <div class="form-control">
                    <label class="label pb-1">
                        <span class="label-text text-xs">{move_tr!("editor-field-fecha-inc")}</span>
                    </label>
                    <input
                        type="date"
                        class="input input-bordered input-xs"
                        prop:value=move || row.with(|r| date_to_html(&r.fecha_incorporacion))
                        on:input=move |ev| upd!(fecha_incorporacion, html_to_date(&input_val(&ev)))
                    />
                </div>

                <div class="form-control">
                    <label class="label pb-1">
                        <span class="label-text text-xs">{move_tr!("editor-field-origen")}</span>
                    </label>
                    <select
                        class="select select-bordered select-xs"
                        prop:value=move || row.with(|r| r.origen.to_string())
                        on:change=move |ev| {
                            let c = select_val(&ev).chars().next().unwrap_or(ORIGEN_ALTA);
                            records.update(|v| {
                                if let Some((_, r)) = v.iter_mut().find(|(i, _)| *i == id) {
                                    r.origen = c;
                                    if c != ORIGEN_CANCELACION { r.fecha_extincion = "00000000".to_string(); }
                                }
                            });
                        }
                    >
                        <option value={ORIGEN_ALTA.to_string()}>{move_tr!("origen-a")}</option>
                        <option value={ORIGEN_MODIFICACION.to_string()}>{move_tr!("origen-m")}</option>
                        <option value={ORIGEN_CANCELACION.to_string()}>{move_tr!("origen-c")}</option>
                    </select>
                </div>

                // Fecha extinción (solo cuando origen = C)
                {move || {
                    let origen = row.with(|r| r.origen);
                    (origen == ORIGEN_CANCELACION).then(|| view! {
                        <div class="form-control">
                            <label class="label pb-1">
                                <span class="label-text text-xs">{move_tr!("editor-field-fecha-ext")}</span>
                            </label>
                            <input
                                type="date"
                                class="input input-bordered input-xs"
                                prop:value=move || row.with(|r| date_to_html(&r.fecha_extincion))
                                on:input=move |ev| upd!(fecha_extincion, html_to_date(&input_val(&ev)))
                            />
                        </div>
                    })
                }}

                <div class="form-control">
                    <label class="label pb-1">
                        <span class="label-text text-xs">{move_tr!("editor-field-val1")}</span>
                    </label>
                    <input
                        type="number" step="0.01" min="0"
                        class="input input-bordered input-xs font-mono"
                        prop:value=move || row.with(|r| format!("{:.2}", r.valoracion1))
                        on:input=move |ev| {
                            let v: f64 = input_val(&ev).parse().unwrap_or(0.0);
                            upd!(valoracion1, v);
                        }
                    />
                </div>

                <div class="form-control">
                    <label class="label pb-1">
                        <span class="label-text text-xs">{move_tr!("editor-field-val2")}</span>
                        <span class="label-text-alt opacity-50">{move_tr!("editor-field-optional")}</span>
                    </label>
                    <input
                        type="number" step="0.01" min="0"
                        class="input input-bordered input-xs font-mono"
                        prop:value=move || row.with(|r| format!("{:.2}", r.valoracion2))
                        on:input=move |ev| {
                            let v: f64 = input_val(&ev).parse().unwrap_or(0.0);
                            upd!(valoracion2, v);
                        }
                    />
                </div>

                <div class="form-control">
                    <label class="label pb-1">
                        <span class="label-text text-xs">{move_tr!("editor-field-pct-participacion")}</span>
                    </label>
                    <label class="input input-bordered input-xs flex items-center gap-1 font-mono">
                        <input
                            type="number" min="0" max="100" step="0.01"
                            class="w-full"
                            prop:value=move || row.with(|r| format!("{:.2}", r.porcentaje_participacion as f64 / 100.0))
                            on:input=move |ev| {
                                let pct: f64 = input_val(&ev).parse().unwrap_or(0.0);
                                let n = (pct.clamp(0.0, 100.0) * 100.0).round() as u32;
                                upd!(porcentaje_participacion, n);
                            }
                        />
                        <span class="opacity-40 text-xs">"%"</span>
                    </label>
                </div>
            </div>
        </div>
    }
}
