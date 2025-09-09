use crate::icons;
use chrono::{DateTime, Duration, Utc};
use dioxus::prelude::*;
use std::fmt;
use wasm_bindgen::JsCast;

#[component]
pub fn DateRangeButton(date_range: Signal<Option<DateRange>>) -> Element {
    let mut is_dropdown_open = use_signal(|| false);

    rsx! {
        div { position: "relative", display: "inline-block",

            button {
                onclick: move |_| is_dropdown_open.toggle(),
                onblur: move |_| {
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
                        } else {
                            is_dropdown_open.set(false);
                        }
                    });
                },
                tabindex: "0",
                r#type: "button",
                letter_spacing: "0.02857em",
                text_transform: "uppercase",
                padding: "6px 16px",
                border_radius: "4px",
                border: "0",
                color: "rgb(255, 255, 255)",
                background_color: "black",
                box_shadow: "none",
                cursor: "pointer",
                outline: "0",
                user_select: "none",
                vertical_align: "middle",
                text_decoration: "none",
                display: "inline-flex",
                align_items: "center",
                justify_content: "center",
                position: "relative",
                margin_right: "30px",

                span {
                    margin_right: "8px",
                    margin_left: "-4px",
                    icons::Clock {}
                }
                span { text_transform: "none",
                    match date_range() {
                        Some(date_range) => date_range.to_string(),
                        None => "No date range".to_string(),
                    }
                }
                span {
                    overflow: "hidden",
                    pointer_events: "none",
                    position: "absolute",
                    z_index: 0,
                    top: 0,
                    right: 0,
                    bottom: 0,
                    left: 0,
                }
            }

            if is_dropdown_open() {
                DateRangeSelector { date_range, is_dropdown_open }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DateRange {
    Last15Min,
    Last30Min,
    Last1Hour,
    Last7Days,
    Last30Days,
    Last3Months,
    LastYear,
    Custom {
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    },
}

impl fmt::Display for DateRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DateRange::Last15Min => write!(f, "Last 15 min"),
            DateRange::Last30Min => write!(f, "Last 30 min"),
            DateRange::Last1Hour => write!(f, "Last 1 hour"),
            DateRange::Last7Days => write!(f, "Last 7 days"),
            DateRange::Last30Days => write!(f, "Last 30 days"),
            DateRange::Last3Months => write!(f, "Last 3 months"),
            DateRange::LastYear => write!(f, "Last year"),
            DateRange::Custom { start, end } => {
                let start_str = start.format("%Y/%m/%d %H:%M:%S").to_string();
                let end_str = end.format("%Y/%m/%d %H:%M:%S").to_string();
                write!(f, "{} - {}", start_str, end_str)
            }
        }
    }
}

impl DateRange {
    pub fn as_str(&self) -> &'static str {
        match self {
            DateRange::Last15Min => "Last 15 min",
            DateRange::Last30Min => "Last 30 min",
            DateRange::Last1Hour => "Last 1 hour",
            DateRange::Last7Days => "Last 7 days",
            DateRange::Last30Days => "Last 30 days",
            DateRange::Last3Months => "Last 3 months",
            DateRange::LastYear => "Last year",
            DateRange::Custom { .. } => "Custom",
        }
    }

    pub fn to_timestamp_nanos(&self) -> i64 {
        let now = Utc::now();

        match self {
            DateRange::Last15Min => (now - Duration::minutes(15)).timestamp(),
            DateRange::Last30Min => (now - Duration::minutes(30)).timestamp(),
            DateRange::Last1Hour => (now - Duration::hours(1)).timestamp(),
            DateRange::Last7Days => (now - Duration::days(7)).timestamp(),
            DateRange::Last30Days => (now - Duration::days(30)).timestamp(),
            DateRange::Last3Months => (now - Duration::days(90)).timestamp(),
            DateRange::LastYear => (now - Duration::days(365)).timestamp(),
            DateRange::Custom { start, end: _ } => (start.timestamp_nanos_opt().unwrap_or(0)),
        }
    }
}

#[component]
fn CustomDateSelector() -> Element {
    let mut start_date = use_signal(|| "".to_string());
    let mut end_date = use_signal(|| "".to_string());

    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            gap: "16px",

            div {
                display: "flex",
                gap: "16px",
                flex_wrap: "wrap",

                div {
                    flex: "1",
                    min_width: "200px",
                    TextField {
                        label: "Start Date".to_string(),
                        value: start_date(),
                        on_change: move |value| start_date.set(value),
                        placeholder: "yyyy/mm/dd hh:mm:ss".to_string(),
                    }
                }

                div {
                    flex: "1",
                    min_width: "200px",
                    TextField {
                        label: "End Date".to_string(),
                        value: end_date(),
                        on_change: move |value| end_date.set(value),
                        placeholder: "yyyy/mm/dd hh:mm:ss".to_string(),
                    }
                }
            }

            div {
                display: "flex",
                justify_content: "flex-start",
                gap: "10px",

                button {
                    border: "1px solid rgb(25, 118, 210)",
                    color: "rgb(25, 118, 210)",
                    background: "white",
                    padding: "6px 16px",
                    border_radius: "4px",
                    cursor: "pointer",
                    font_size: "14px",
                    text_transform: "uppercase",
                    letter_spacing: "0.02857em",
                    min_width: "64px",
                    onclick: move |_| {
                        println!("Reset clicked");
                    },
                    "Reset"
                }

                button {
                    background: "rgb(25, 118, 210)",
                    color: "white",
                    border: "none",
                    padding: "6px 16px",
                    border_radius: "4px",
                    cursor: "pointer",
                    font_size: "14px",
                    text_transform: "uppercase",
                    letter_spacing: "0.02857em",
                    min_width: "64px",
                    onclick: move |_| {
                        println!("Apply clicked - Start: {}, End: {}", start_date(), end_date());
                    },
                    "Apply"
                }
            }
        }
    }
}

#[component]
fn TextField(
    label: String,
    value: String,
    on_change: EventHandler<String>,
    placeholder: String,
) -> Element {
    rsx! {
        div {
            position: "relative",

            label {
                position: "absolute",
                top: "0",
                left: "0",
                transform: "translate(14px, -6px) scale(0.75)",
                background_color: "transparent",
                padding: "0 4px",
                color: "#959493",
                pointer_events: "none",
                z_index: 1,
                "{label}"
            }

            div {
                position: "relative",
                display: "flex",
                align_items: "center",
                border: "1px solid rgba(0, 0, 0, 0.23)",
                border_radius: "4px",
                padding: "16.5px 14px",
                background_color: "transparent",

                input {
                    r#type: "tel",
                    value: "{value}",
                    placeholder: "{placeholder}",
                    border: "none",
                    outline: "none",
                    background: "transparent",

                    oninput: move |event| on_change.call(event.value()),
                }

                div {
                    display: "flex",
                    align_items: "center",
                    margin_left: "8px",

                    button {
                        background: "none",
                        border: "none",
                        cursor: "pointer",
                        padding: "4px",
                        border_radius: "50%",
                        display: "flex",
                        align_items: "center",
                        justify_content: "center",

                        onclick: move |_| {
                            println!("Open date picker for {label}");
                        },

                        icons::Calendar {}
                    }
                }
            }
        }
    }
}

#[component]
fn DateRangeSelector(
    date_range: Signal<Option<DateRange>>,
    is_dropdown_open: Signal<bool>,
) -> Element {
    //let mut custom_dates_selection = use_signal(|| false);

    let periods = vec![
        Some(DateRange::Last15Min),
        Some(DateRange::Last30Min),
        Some(DateRange::Last1Hour),
        Some(DateRange::Last7Days),
        Some(DateRange::Last30Days),
        Some(DateRange::Last3Months),
        Some(DateRange::LastYear),
        None,
    ];

    rsx! {
        div {
            position: "absolute",
            top: "100%",
            right: 0,
            display: "flex",
            background_color: "white",
            border_radius: "4px",
            box_shadow: "0px 5px 5px -3px rgba(0,0,0,0.2), 0px 8px 10px 1px rgba(0,0,0,0.14), 0px 3px 14px 2px rgba(0,0,0,0.12)",
            opacity: "1",
            transform: "none",
            z_index: 1300,

            div {
                //border_right: if custom_dates_selection() { "1px solid rgba(0, 0, 0, 0.12)" } else { "none" },

                div { padding: "8px 16px",

                    div { "Select a period" }

                    hr {
                        margin: "8px 0",
                        border: "none",
                        height: "1px",
                        background_color: "rgba(0, 0, 0, 0.12)",
                    }

                    div {
                        ul { list_style: "none", padding: "0", margin: "0",
                            for range_value in periods {
                                li {
                                    key: match range_value {
                                        Some(ref date_range) => date_range.to_string(),
                                        None => "Reset".to_string(),
                                    },
                                    button {
                                        padding: "8px 16px",
                                        border: "none",
                                        background: "none",
                                        cursor: "pointer",
                                        text_align: "left",
                                        display: "flex",
                                        align_items: "center",

                                        onclick: move |_| {
                                            is_dropdown_open.set(false);
                                            date_range.set(range_value.clone());
                                        },

                                        div { flex: "1 1 auto",
                                            match range_value {
                                                Some(ref date_range) => date_range.to_string(),
                                                None => "Reset".to_string(),
                                            }
                                        }

                                        span {
                                            overflow: "hidden",
                                            pointer_events: "none",
                                            position: "absolute",
                                            top: "0",
                                            right: "0",
                                            bottom: "0",
                                            left: "0",
                                        }
                                    }
                                }
                            }

                            //li {
                            //    button {
                            //        padding: "8px 16px",
                            //        border: "none",
                            //        background: "none",
                            //        cursor: "pointer",
                            //        text_align: "left",
                            //        display: "flex",
                            //        align_items: "center",

                            //        onblur: move |_| {
                            //            spawn(async move {
                            //                gloo_timers::future::TimeoutFuture::new(200).await;
                            //                if let Some(window) = web_sys::window()
                            //                    && let Some(document) = window.document()
                            //                    && let Some(active_element) = document.active_element()
                            //                    && let Some(element) = active_element.dyn_ref::<web_sys::HtmlElement>()
                            //                {
                            //                    let tag_name = element.tag_name().to_lowercase();
                            //                    if tag_name != "input" && tag_name != "button" {
                            //                        is_dropdown_open.set(false);
                            //                    }
                            //                } else {
                            //                    is_dropdown_open.set(false);
                            //                }
                            //            });
                            //        },
                            //        onclick: move |_| {
                            //            custom_dates_selection.set(!custom_dates_selection());
                            //        },

                            //        div {
                            //            display: "flex",
                            //            align_items: "center",
                            //            margin_right: "8px",
                            //            icons::DateRange {}
                            //        }

                            //        div { flex: "1 1 auto", "Custom dates" }

                            //        div {
                            //            display: "flex",
                            //            align_items: "center",
                            //            margin_left: "8px",
                            //            icons::ChevronRight {}
                            //        }

                            //        span {
                            //            overflow: "hidden",
                            //            pointer_events: "none",
                            //            position: "absolute",
                            //            top: "0",
                            //            right: "0",
                            //            bottom: "0",
                            //            left: "0",
                            //        }
                            //    }
                            //}
                        }
                    }
                }
            }

            //if custom_dates_selection() {
            //    div {
            //        padding: "16px",
            //        CustomDateSelector {  }
            //    }
            //}
        }
    }
}
