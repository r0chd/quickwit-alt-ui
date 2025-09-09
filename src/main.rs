#![allow(non_snake_case)]
mod icons;
mod indexes;
mod query_editor;
mod quickwit;

use crate::quickwit::QuickwitApi;
use dioxus::prelude::*;
use dioxus_logger::tracing::{Level, info};
use indexes::Indexes;
use query_editor::QueryEditor;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[layout(NavLayout)]
    #[route("/")]
    #[redirect("/", ||Route::QueryEditor)]
    Home,
    #[route("/search")]
    QueryEditor,
    #[route("/indexes")]
    Indexes,
}

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

struct FontSpec {
    asset: Asset,
    weight: u16,
}

const FONT_SPECS: [FontSpec; 4] = [
    FontSpec {
        asset: asset!("assets/fonts/soehne-web-buch.woff2"),
        weight: 300,
    },
    FontSpec {
        asset: asset!("assets/fonts/soehne-mono-web-kraftig.woff2"),
        weight: 500,
    },
    FontSpec {
        asset: asset!("assets/fonts/soehne-web-halbfett.woff2"),
        weight: 600,
    },
    FontSpec {
        asset: asset!("assets/fonts/soehne-mono-web-dreiviertelfett.woff2"),
        weight: 700,
    },
];

fn font_face(spec: &FontSpec) -> String {
    format!(
        r#"
        @font-face {{
          font-family: 'SoehneMono';
          src: url({}) format("woff2");
          font-weight: {};
        }}
        "#,
        spec.asset, spec.weight
    )
}

fn App() -> Element {
    let theme_css = asset!("assets/dx-components-theme.css");

    rsx! {
        document::Stylesheet { href: theme_css }
        for spec in FONT_SPECS {
            document::Link { href: spec.asset }
            style { "{font_face(&spec)}" }
        }
        style { {include_str!("../assets/styles.css")} }
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {}
}

#[derive(serde::Deserialize, Debug)]
struct Cluster {
    cluster_id: String,
}

fn Header() -> Element {
    let data = use_resource(|| async { QuickwitApi::cluster().await.unwrap() });

    rsx! {
        header {
            height: "50px",
            background_color: "#1b1d1e",
            color: "#fff",
            display: "flex",
            align_items: "center",
            padding: "0 16px",
            font_weight: "bold",
            justify_content: "space-between",
            if let Some(data) = data.read().as_ref() {
                p { "{data.cluster_id}" }
            }
            div { display: "flex", gap: "25px", margin_right: "20px",
                a {
                    href: "https://quickwit.io/docs",
                    text_decoration: "underline",
                    text_decoration_color: "#5F6060",
                    "Docs"
                }
                a { href: "https://discord.gg/rpRRTezWhW", icons::Discord {} }
                a { href: "https://github.com/quickwit-inc/quickwit", icons::GitHub {} }
            }
        }
    }
}

#[component]
pub fn NavLayout() -> Element {
    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            class: "layout-container",
            Header {}
            div {
                display: "flex",
                flex: "1",
                border_left: "1px solid #8c82731f",
                nav { border_right: "1px solid #8c82731f", width: "180px",
                    div {
                        NavLink {
                            to: Route::QueryEditor {},
                            icon: rsx! {
                                icons::Code {}
                            },
                            text: "Query editor",
                        }
                        NavLink {
                            to: Route::Indexes {},
                            icon: rsx! {
                                icons::Database {}
                            },
                            text: "Indexes",
                        }
                    }
                }
                div {
                    class: "content-area",
                    flex: 1,
                    display: "flex",
                    flex_direction: "column",
                    overflow: "hidden",
                    Outlet::<Route> {}
                }
            }
        }
    }
}

#[component]
fn NavLink(to: Route, icon: Element, text: &'static str) -> Element {
    rsx! {
        Link { to, class: "nav-link",
            div { class: "nav-icon", {icon} }
            span { "{text}" }
        }
    }
}
