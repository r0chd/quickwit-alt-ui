use crate::icons;
use crate::quickwit::QuickwitApi;
use crate::quickwit::index::Index;
use chrono::{TimeZone, Utc};
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use wasm_bindgen::prelude::*;

#[component]
fn IndexInput(search_value: Signal<String>, selected_index: Signal<Option<String>>) -> Element {
    let mut is_dropdown_open = use_signal(|| false);
    let mut is_hovered = use_signal(|| false);
    let data = use_resource(|| async { QuickwitApi::indexes().await.unwrap() });

    let filtered_indexes = move || -> Vec<_> {
        data.read()
            .as_ref()
            .map(|indexes| {
                indexes
                    .iter()
                    .filter(|index| {
                        index
                            .index_config
                            .index_id
                            .to_lowercase()
                            .contains(&search_value().to_lowercase())
                            || selected_index.read().is_some()
                    })
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    };

    let handle_blur = move |_| {
        spawn(async move {
            gloo_timers::future::TimeoutFuture::new(200).await;
            if let Some(window) = web_sys::window()
                && let Some(document) = window.document()
                && let Some(active_element) = document.active_element()
                && let Some(element) = active_element.dyn_ref::<web_sys::HtmlElement>()
            {
                let tag_name = element.tag_name().to_lowercase();
                if tag_name != "input" && tag_name != "button" {
                    is_dropdown_open.set(false);
                }

                if filtered_indexes()
                    .iter()
                    .all(|index| index.index_config.index_id != search_value())
                {
                    search_value.set(selected_index().unwrap_or_default());
                }
            } else {
                is_dropdown_open.set(false);
            }
        });
    };

    rsx! {
        div {
            onmouseenter: move |_| {
                is_hovered.set(true);
            },
            onmouseleave: move |evt| {
                is_hovered.set(false);
            },
            border: "1px solid #323230",
            border_radius: "4px",
            background: "transparent",
            display: "flex",
            align_items: "center",
            max_width: "300px",
            input {
                flex: "1 1 0",
                min_width: 0,
                background: "transparent",
                border: "none",
                color: "#f8f9fa",
                outline: "none",
                padding: "10px",
                placeholder: "Select an index",
                value: "{search_value}",
                oninput: move |evt| {
                    search_value.set(evt.value());
                    if filtered_indexes()
                        .iter()
                        .any(|index| { index.index_config.index_id == search_value() })
                    {
                        selected_index.set(Some(search_value()));
                    }
                },
                onblur: handle_blur,
                onfocus: move |_| is_dropdown_open.set(true),
            }
            if !search_value.read().is_empty() && is_hovered() {
                button {
                    background: "transparent",
                    border: "none",
                    color: "#e8e6e38a",
                    padding: "4px 8px",
                    cursor: "pointer",
                    display: "flex",
                    align_items: "center",
                    justify_content: "center",
                    flex_shrink: 0,
                    onclick: move |_| {
                        selected_index.set(None);
                        search_value.set(String::new());
                    },
                    icons::Close {}
                }
            }
            button {
                background: "transparent",
                border: "none",
                color: "#e8e6e38a",
                padding: "4px 8px",
                cursor: "pointer",
                display: "flex",
                align_items: "center",
                justify_content: "center",
                flex_shrink: 0,
                onblur: handle_blur,
                onfocus: move |_| is_dropdown_open.set(true),
                onclick: move |_| is_dropdown_open.toggle(),
                if is_dropdown_open() {
                    icons::FilledArrowUp {}
                } else {
                    icons::FilledArrowDown {}
                }
            }
        }
        div { position: "relative", z_index: 1,
            if is_dropdown_open() {
                IndexDropdown {
                    indexes: filtered_indexes(),
                    on_select: move |index_id: String| {
                        search_value.set(index_id);
                        if filtered_indexes()
                            .iter()
                            .any(|index| index.index_config.index_id == search_value())
                        {
                            selected_index.set(Some(search_value()));
                        } else {
                            selected_index.set(None);
                        }
                        is_dropdown_open.set(false);
                    },
                    is_loading: data.read().is_none(),
                }
            }
        }
    }
}

#[component]
pub fn IndexSelector(
    search_value: Signal<String>,
    selected_index: Signal<Option<String>>,
) -> Element {
    rsx! {
        div {
            display: "flex",
            flex: "0 0 260px",
            flex_direction: "column",
            height: "100vh",
            border_right: "1px solid #8c82731f",
            max_width: "260px",
            div {
                padding_top: "18px",
                padding_left: "24px",
                padding_right: "24px",
                p { margin_bottom: "10px", "Index ID" }
                IndexInput { search_value, selected_index }
            }
            FieldsPanel { selected_index_id: selected_index() }
        }
    }
}

#[component]
fn IndexDropdown(
    indexes: Vec<Index>,
    on_select: EventHandler<String>,
    is_loading: bool,
) -> Element {
    rsx! {
        div {
            position: "fixed",
            width: "200px",
            border_radius: "4px",
            max_height: "200px",
            overflow_y: "auto",
            box_shadow: "0 4px 6px rgba(0, 0, 0, 0.3)",

            for idx in indexes {
                div {
                    key: "{idx.index_config.index_id}",
                    padding: "12px 16px",
                    cursor: "pointer",
                    color: "#f8f9fa",
                    background_color: "transparent",
                    overflow: "hidden",
                    onclick: {
                        let index_id = idx.index_config.index_id.clone();
                        move |_| {
                            on_select.call(index_id.clone());
                        }
                    },
                    div { width: "300px", "{idx.index_config.index_id}" }
                }
            }
        }
    }
}

#[component]
fn FieldsPanel(selected_index_id: Option<String>) -> Element {
    let mut is_fields_expanded = use_signal(|| false);
    let data = use_resource(|| async { QuickwitApi::indexes().await.unwrap() });

    let get_selected_index = || -> Option<Index> {
        data.read()
            .as_ref()
            .and_then(|indexes| {
                indexes.iter().find(|index| {
                    selected_index_id
                        .as_ref()
                        .map_or(false, |id| id == &index.index_config.index_id)
                })
            })
            .cloned()
    };

    rsx! {
        div {
            flex: 1,
            display: "flex",
            flex_direction: "column",
            padding: "0 24px",
            div {
                display: "flex",
                align_items: "center",
                padding_top: "12px",
                cursor: "pointer",
                flex_shrink: 0,
                onclick: move |_| is_fields_expanded.toggle(),
                button {
                    width: "auto",
                    border: "none",
                    color: "#f8f9fa",
                    cursor: "pointer",
                    display: "flex",
                    align_items: "center",
                    transition: "transform 0.2s",
                    background_color: "transparent",
                    margin_right: "8px",
                    padding: 0,
                    if is_fields_expanded() {
                        icons::ArrowRight {}
                    } else {
                        icons::ArrowDown {}
                    }
                }
                span { color: "#f8f9fa", "Fields" }
            }

            if is_fields_expanded() {
                div { flex: 1,
                    ul {
                        list_style: "none",
                        padding: "0",
                        margin: "8px 0",
                        overflow_wrap: "break-word",
                        if let Some(selected_index) = get_selected_index() {
                            {
                                selected_index
                                    .index_config
                                    .doc_mapping
                                    .field_mappings
                                    .iter()
                                    .map(|field| {
                                        rsx! {
                                            li { padding: "4px 0", color: "#f8f9fa", "{field.name}" }
                                        }
                                    })
                            }
                        }
                    }
                }
            }
        }
    }
}
