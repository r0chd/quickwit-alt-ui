mod index_selector;
mod time_range_select;

use crate::document::eval;
use crate::icons;
use crate::quickwit::QuickwitApi;
use crate::quickwit::query::QueryResponse;
use chrono::{DateTime, TimeZone, Utc};
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use index_selector::IndexSelector;
use num_format::{Locale, ToFormattedString};
use time_range_select::DateRange;
use wasm_bindgen::prelude::*;

#[component]
pub fn CollapseAllCheckbox(collapse_all: Signal<bool>) -> Element {
    rsx! {
        div {
            display: "flex",
            align_items: "center",
            margin_right: "30px",
            label {
                display: "flex",
                align_items: "center",
                cursor: "pointer",
                color: "#5F6060",
                font_size: "14px",
                font_weight: "500",
                padding: "8px 4px",
                border_radius: "4px",
                transition: "all 0.2s ease",

                div {
                    position: "relative",
                    margin_right: "12px",
                    width: "20px",
                    height: "20px",

                    input {
                        r#type: "checkbox",
                        position: "absolute",
                        opacity: "0",
                        width: "100%",
                        height: "100%",
                        cursor: "pointer",
                        margin: "0",
                        z_index: "1",

                        checked: collapse_all(),
                        onclick: move |_| { collapse_all.toggle() }
                    }

                    div {
                        position: "absolute",
                        top: "0",
                        left: "0",
                        width: "20px",
                        height: "20px",
                        border: "2px solid #d1d5db",
                        border_radius: "4px",
                        background_color: if collapse_all() { "rgb(25, 118, 210)" } else { "white" },
                        border_color: if collapse_all() { "rgb(25, 118, 210)" } else { "#d1d5db" },
                        transition: "all 0.2s ease",
                        display: "flex",
                        align_items: "center",
                        justify_content: "center",

                        if collapse_all() {
                            svg {
                                width: "14px",
                                height: "14px",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "white",
                                stroke_width: "3",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path {
                                    d: "M20 6L9 17l-5-5"
                                }
                            }
                        }
                    }
                }
                "Collapse all"
            }
        }
    }
}

#[component]
pub fn MaxHitsSelector(max_hits: Signal<usize>) -> Element {
    rsx! {
        div { display: "flex", align_items: "center", margin_right: "30px",
            label { color: "#5F6060", font_size: "14px", margin_right: "8px", "Max hits:" }
            input {
                r#type: "number",
                background_color: "#2d2d2d",
                color: "#f8f9fa",
                border: "1px solid #404040",
                border_radius: "4px",
                padding: "6px 12px",
                outline: "none",
                width: "80px",
                min: "1",
                max: "1000",
                value: "{max_hits()}",
                oninput: move |e| {
                    if let Ok(value) = e.value().parse::<usize>() {
                        if value > 0 && value <= 1000 {
                            max_hits.set(value);
                        }
                    }
                },
            }
        }
    }
}

#[component]
pub fn RunButton(
    query: String,
    query_results: Signal<QueryResponse>,
    selected_index: Signal<Option<String>>,
    max_hits: Signal<usize>,
    date_range: Signal<Option<DateRange>>,
) -> Element {
    rsx! {
        div { flex_grow: 1, margin_bottom: "6px",
            button {
                border: "0",
                color: if selected_index.read().is_none() { "#4C4D4D" } else { "white" },
                background_color: if selected_index.read().is_none() { "#151718" } else { "black" },
                display: "inline-flex",
                align_items: "center",
                justify_content: "center",
                position: "relative",
                cursor: if selected_index.read().is_none() { "not-allowed" } else { "pointer" },
                vertical_align: "middle",
                padding: "8px 16px",
                border_radius: "4px",
                disabled: selected_index.read().is_none(),
                onclick: move |_| {
                    if let Some(selected_index) = selected_index.read().clone() {
                        let query = query.clone();
                        spawn(async move {
                            let date_range_str = match date_range() {
                                Some(date_range) => format!("{}", date_range.to_timestamp_nanos()),
                                None => "".to_string(),
                            };
                            let results = QuickwitApi::query(&selected_index)
                                .max_hits(max_hits())
                                .sort_by_field("timestamp_nanos")
                                .start_timestamp(&date_range_str)
                                .end_timestamp("")
                                .execute()
                                .await
                                .unwrap();
                            query_results.set(results);
                        });
                    }
                },
                span {
                    display: "flex",
                    align_items: "center",
                    margin_right: "8px",
                    margin_left: "-4px",
                    icons::FilledArrowRight {}
                }
                "RUN"
            }
        }
    }
}

#[component]
pub fn QueryEditor() -> Element {
    let mut query = use_signal(|| "*".to_string());
    let mut query_results = use_signal(|| QueryResponse::default());
    let mut selected_index = use_signal(|| None);
    let mut max_hits = use_signal(|| 20);
    let mut search_value = use_signal(|| String::new());
    let collapse_all = use_signal(|| true);
    let date_range: Signal<Option<DateRange>> = use_signal(|| None);

    use_effect(move || {
        if let Some(window) = web_sys::window() {
            if let Ok(url) = window.location().href() {
                if let Ok(parsed_url) = url::Url::parse(&url) {
                    let params: std::collections::HashMap<_, _> =
                        parsed_url.query_pairs().collect();

                    if let Some(q) = params.get("query") {
                        query.set(q.to_string());
                    }

                    if let Some(hits_str) = params.get("max_hits") {
                        if let Ok(hits) = hits_str.parse::<usize>() {
                            max_hits.set(hits);
                        }
                    }

                    if let Some(idx) = params.get("index") {
                        search_value.set(idx.to_string());
                        selected_index.set(Some(idx.to_string()));
                    }

                    spawn(async move {
                        let date_range_str = match date_range() {
                            Some(date_range) => "1758470661".to_string(),
                            None => "".to_string(),
                        };
                        info!("{date_range_str}");
                        if let Some(selected_index) = selected_index() {
                            let results = QuickwitApi::query(&selected_index)
                                .max_hits(max_hits())
                                .sort_by_field("timestamp_nanos")
                                .start_timestamp(&date_range_str)
                                .end_timestamp("")
                                .execute()
                                .await
                                .unwrap();
                            query_results.set(results);
                        }
                    });
                }
            }
        }
    });

    rsx! {
        div { width: "100%", display: "flex", height: "100vh",
            IndexSelector { search_value, selected_index }
            div {
                display: "flex",
                flex_direction: "column",
                flex: 1,
                padding: "24px",
                div { padding_bottom: "5px", display: "flex", gap: "8px",
                    RunButton {
                        query: query(),
                        query_results,
                        selected_index,
                        max_hits,
                        date_range,
                    }
                    MaxHitsSelector { max_hits }
                    CollapseAllCheckbox { collapse_all }
                    time_range_select::DateRangeButton {
                        date_range,
                    }
                }
                textarea {
                    value: "{query()}",
                    oninput: move |e| query.set(e.value()),
                    width: "100%",
                    min_height: "100px",
                    height: "100px",
                    max_height: "100px",
                    background_color: "#2d2d2d",
                    color: "#f8f9fa",
                    border: "1px solid #404040",
                    resize: "none",
                    padding: "5px",
                    border_radius: "4px",
                    margin_bottom: "12px",
                }
                HitCount { query_results: query_results.read().clone() }
                div { style: "flex: 1; min-height: 0; overflow-y: auto;",
                    ResultTable {
                        hits: query_results.read().hits.clone(),
                        collapse_all,
                    }
                }
            }
        }
    }
}

#[component]
fn HitCount(query_results: Option<QueryResponse>) -> Element {
    rsx! {
        p { color: "#5F6060", margin_bottom: "10px",
            {
                format!(
                    "{} hits found in {:.2} seconds",
                    query_results
                        .as_ref()
                        .map(|results| results.num_hits)
                        .unwrap_or_default()
                        .to_formatted_string(&Locale::en),
                    query_results
                        .as_ref()
                        .map(|results| results.elapsed_time_micros)
                        .unwrap_or_default() / 1_000_000.0,
                )
            }
        }
    }
}

fn timestamp_nanos_u64_to_date_parts(timestamp_nanos: i64) -> (String, String) {
    let datetime: DateTime<Utc> = Utc.timestamp_nanos(timestamp_nanos);
    let date_part = datetime.format("%Y/%m/%d").to_string();
    let time_part = datetime.format("%H:%M:%S").to_string();
    (date_part, time_part)
}

#[component]
fn ResultTable(hits: Vec<serde_json::Value>, collapse_all: Signal<bool>) -> Element {
    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            overflow_y: "auto",
            flex_grow: 1,
            max_height: "calc(100% - 50px)",
            for hit in hits.iter() {
                {
                    let mut collapsed = use_signal(|| collapse_all());
                    use_effect(move || {
                        collapsed.set(collapse_all());
                    });

                    let (date_part, time_part) = timestamp_nanos_u64_to_date_parts(1000);

                    rsx! {
                        div { style: "display: flex; align-items: flex-start; border-bottom: 1px solid #333;",
                            div {
                                padding: "8px 8px 8px 0",
                                display: "flex",
                                flex_direction: "column",
                                align_items: "flex-start",
                                cursor: "pointer",
                                min_width: "120px",
                                flex_shrink: "0",
                                onclick: move |_| {
                                    collapsed.toggle();
                                },
                                div {
                                    display: "flex",
                                    align_items: "center",
                                    svg {
                                        width: "16px",
                                        height: "16px",
                                        view_box: "0 0 24 24",
                                        fill: "currentColor",
                                        if collapsed() {
                                            path { d: "M10 6L8.59 7.41 13.17 12l-4.58 4.59L10 18l6-6z" }
                                        } else {
                                            path { d: "M7.41 8.59L12 13.17l4.59-4.58L18 10l-6 6-6-6 1.41-1.41z" }
                                        }
                                    }
                                    div {
                                        display: "flex",
                                        flex_direction: "column",
                                        margin_left: "8px",
                                        p {
                                            margin: "0",
                                            line_height: "1.2",
                                            font_size: "12px",
                                            "{date_part}"
                                        }
                                        p {
                                            margin: "0",
                                            line_height: "1.2",
                                            font_size: "12px",
                                            "{time_part}"
                                        }
                                    }
                                }
                            }
                            div { style: "padding: 8px; word-break: break-all; flex-grow: 1;",
                                if let serde_json::Value::Object(obj) = hit {
                                    if collapsed() {
                                        for (k , value) in obj.iter() {
                                            HitField { k, value: value.clone() }
                                        }
                                    } else {
                                        Json { value: hit.clone() }
                                    }
                                } else {
                                    if collapsed() {
                                        div { style: "color: #f8f9fa;", "{hit}" }
                                    } else {
                                        Json { value: hit.clone() }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Json(value: serde_json::Value) -> Element {
    let json_string = serde_json::to_string_pretty(&value).unwrap();

    use_effect(move || {
        spawn(async move {
            let _ = eval(
                r#"
                setTimeout(() => {
                    if (window.hljs) {
                        hljs.highlightAll();
                    }
                }, 0)
                "#,
            )
            .await;
        });
    });

    rsx! {
        pre { padding: "12px", border_radius: "4px",
            code { class: "language-json", dangerous_inner_html: "{json_string}" }
        }
    }
}

#[component]
fn HitField(k: String, value: serde_json::Value) -> Element {
    rsx! {
        span { style: "font-size: 12px; padding: 2px; background-color: #242728; color: #B5AFA7; font-weight: bold; margin-right: 4px;",
            "{k}:"
        }
        span { style: "font-size: 12px; color: #B5AFA7; font-family: monospace; margin-right: 4px;",
            match value {
                serde_json::Value::String(s) => rsx! {
                "{s}"
                },
                serde_json::Value::Number(n) => rsx! {
                "{n}"
                },
                serde_json::Value::Bool(b) => rsx! {
                "{b}"
                },
                serde_json::Value::Null => rsx! { "null" },
                _ => rsx! {
                "{value}"
                },
            }
        }
    }
}
