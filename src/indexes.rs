use crate::{Route, icons, quickwit};
use chrono::{NaiveDateTime, TimeZone, Utc};
use dioxus::prelude::*;
use quickwit::index::{Index, SplitState};

#[component]
fn IndexRow(index: quickwit::index::Index) -> Element {
    let mut is_expanded = use_signal(|| false);

    rsx! {
        tr {
            style: "border-bottom: 1px solid #404040; transition: background-color 0.2s ease; cursor: pointer;",
            onmouseenter: |_| {},
            onmouseleave: |_| {},
            onclick: move |_| {
                is_expanded.set(!is_expanded());
            },
            td { style: "padding: 12px 8px; text-align: center; color: #f8f9fa; font-size: 12px; user-select: none;",
                if is_expanded() {
                    icons::FilledArrowDown {}
                } else {
                    icons::FilledArrowRight {}
                }
            }
            th { style: "padding: 12px 16px; font-weight: 500; color: #f8f9fa; font-size: 13px; text-align: left;",
                "{index.index_config.index_id}"
            }
            td { style: "padding: 12px 16px; font-weight: 500; color: #f8f9fa; font-size: 13px; text-align: left;",
                "{index.index_config.index_uri}"
            }
            td { style: "padding: 12px 16px; font-weight: 500; color: #f8f9fa; font-size: 13px; text-align: left;",
                "{format_utc(index.create_timestamp)}"
            }
            td { style: "padding: 12px 16px; font-weight: 500; color: #f8f9fa; font-size: 13px; text-align: left;",
                "{index.sources.len()}"
            }
        }

        if is_expanded() {
            IndexDetailsRow { index: index.clone() }
        }
    }
}

#[component]
fn IndexDetailsRow(index: Index) -> Element {
    let index_id = index.index_config.index_id.clone();

    let data = use_resource({
        let index_id_clone = index_id.clone();
        move || {
            let value = index_id_clone.clone();
            async move { quickwit::QuickwitApi::index_describe(&value).await.unwrap() }
        }
    });

    let indexing =
        use_resource(move || async move { quickwit::QuickwitApi::indexing().await.unwrap() });

    let splits = use_resource({
        let index_id_clone = index_id.clone();
        move || {
            let value = index_id_clone.clone();
            async move { quickwit::QuickwitApi::index_splits(&value).await.unwrap() }
        }
    });

    rsx! {
        tr { style: "border-bottom: 1px solid #404040; background: #2a2a2a;",
            td { colspan: "5", style: "padding: 0; color: #f8f9fa;",
                table { style: "width: 100%; border-collapse: collapse; font-size: 13px;",
                    tbody {
                        tr { style: "background-color: #1e1e1e;",
                            td { style: "padding: 8px 12px; color: #b0b0b0; width: 250px;",
                                "Created at:"
                            }
                            td { style: "padding: 8px 12px; color: #f8f9fa;",
                                "{format_utc(index.create_timestamp)}"
                            }
                        }
                        tr { style: "background-color: #2a2a2a;",
                            td { style: "padding: 8px 12px; color: #b0b0b0;", "URI:" }
                            td { style: "padding: 8px 12px; color: #f8f9fa; word-break: break-all;",
                                "{index.index_config.index_uri}"
                            }
                        }
                        tr { style: "background-color: #1e1e1e;",
                            td { style: "padding: 8px 12px; color: #b0b0b0;",
                                "Number of published documents:"
                            }
                            td { style: "padding: 8px 12px; color: #f8f9fa;",
                                match data() {
                                    Some(desc) => rsx! {
                                    "{desc.num_published_docs}"
                                    },
                                    None => rsx! { "Loading..." },
                                }
                            }
                        }
                        tr { style: "background-color: #2a2a2a;",
                            td { style: "padding: 8px 12px; color: #b0b0b0;",
                                "Size of published documents (uncompressed):"
                            }
                            td { style: "padding: 8px 12px; color: #f8f9fa;",
                                match data() {
                                    Some(desc) => rsx! {
                                    "{format_bytes(desc.size_published_docs_uncompressed)}"
                                    },
                                    None => rsx! { "Loading..." },
                                }
                            }
                        }
                        tr { style: "background-color: #1e1e1e;",
                            td { style: "padding: 8px 12px; color: #b0b0b0;",
                                "Number of published splits:"
                            }
                            td { style: "padding: 8px 12px; color: #f8f9fa;",
                                match data() {
                                    Some(desc) => rsx! {
                                    "{desc.num_published_splits}"
                                    },
                                    None => rsx! { "Loading..." },
                                }
                            }
                        }
                        tr { style: "background-color: #2a2a2a;",
                            td { style: "padding: 8px 12px; color: #b0b0b0;",
                                "Size of published splits:"
                            }
                            td { style: "padding: 8px 12px; color: #f8f9fa;",
                                match data() {
                                    Some(desc) => rsx! {
                                    "{format_bytes(desc.size_published_splits)}"
                                    },
                                    None => rsx! { "Loading..." },
                                }
                            }
                        }
                        tr { style: "background-color: #1e1e1e;",
                            td { style: "padding: 8px 12px; color: #b0b0b0;",
                                "Number of staged splits:"
                            }
                            td { style: "padding: 8px 12px; color: #f8f9fa;",
                                match indexing() {
                                    Some(indexing_data) => rsx! {
                                    "{indexing_data.num_staged_splits.unwrap_or_default()}"
                                    },
                                    None => rsx! { "Loading..." },
                                }
                            }
                        }
                        tr { style: "background-color: #2a2a2a;",
                            td { style: "padding: 8px 12px; color: #b0b0b0;",
                                "Number of splits marked for deletion:"
                            }
                            td { style: "padding: 8px 12px; color: #f8f9fa;",
                                match splits() {
                                    Some(splits) => rsx! {
                                    "{splits.splits.iter().filter(|split| split.split_state == SplitState::MarkedForDeletion).count()}"
                                    },
                                    None => rsx! { "Loading..." },
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
pub fn Indexes() -> Element {
    let data = use_resource(|| async { quickwit::QuickwitApi::indexes().await.unwrap() });

    let navigator = use_navigator();

    rsx! {
        div { height: "100%", padding: "20px",
            h1 { style: "font-size: 14px; font-weight: 700; margin-bottom: 16px; color: #f8f9fa;",
                "Indexes"
            }
            div { style: "background: #1e1e1e; box-shadow: 0 1px 3px rgba(0,0,0,0.3);",
                table { style: "width: 100%; border-collapse: collapse; background: #181a1b;",
                    thead {
                        tr { style: "background: #181a1b; border-bottom: 1px solid #404040;",
                            th { style: "width: 40px; padding: 12px 8px; text-align: center; font-weight: 600; color: #f8f9fa; font-size: 13px;",
                                ""
                            }
                            th { style: "padding: 12px 16px; text-align: left; font-weight: 600; color: #f8f9fa; font-size: 13px;",
                                "ID"
                            }
                            th { style: "padding: 12px 16px; text-align: left; font-weight: 600; color: #f8f9fa; font-size: 13px;",
                                "URI"
                            }
                            th { style: "padding: 12px 16px; text-align: left; font-weight: 600; color: #f8f9fa; font-size: 13px;",
                                "Created on"
                            }
                            th { style: "padding: 12px 16px; text-align: left; font-weight: 600; color: #f8f9fa; font-size: 13px;",
                                "Sources"
                            }
                        }
                    }
                    tbody {
                        {
                            match data.read().as_ref() {
                                Some(indexes) => rsx! {
                                    for index in indexes {
                                        IndexRow { index: index.clone() }
                                    }
                                },
                                _ => rsx! {
                                    tr {
                                        td { colspan: "5", style: "padding: 12px 16px; color: #b0b0b0;", "Loading..." }
                                    }
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}

fn format_bytes(bytes: u64) -> String {
    if bytes == 0 {
        return "0 MB".to_string();
    }

    let mb = bytes as f64 / (1024.0 * 1024.0);
    if mb < 1.0 {
        let kb = bytes as f64 / 1024.0;
        if kb < 1.0 {
            format!("{} B", bytes)
        } else {
            format!("{:.1} KB", kb)
        }
    } else if mb < 1024.0 {
        format!("{:.1} MB", mb)
    } else {
        let gb = mb / 1024.0;
        format!("{:.1} GB", gb)
    }
}

fn format_utc(timestamp: i64) -> String {
    Utc.from_utc_datetime(&NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap())
        .format("%Y/%m/%d %H:%M")
        .to_string()
}
