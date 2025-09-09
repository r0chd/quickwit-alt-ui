use dioxus::{html::u::background_color, prelude::*};

#[component]
pub fn Code() -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 24",
            width: "18",
            height: "18",
            fill: "currentColor",
            path { d: "m24 12-5.657 5.657-1.414-1.414L21.172 12l-4.243-4.243 1.414-1.414L24 12zM2.828 12l4.243 4.243-1.414 1.414L0 12l5.657-5.657L7.07 7.757 2.828 12zm6.96 9H7.66l6.552-18h2.128L9.788 21z" }
        }
    }
}

#[component]
pub fn Database() -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 24",
            width: "18",
            height: "18",
            fill: "none",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            ellipse {
                cx: "12",
                cy: "5",
                rx: "9",
                ry: "3",
            }
            path { d: "M21 12c0 1.66-4 3-9 3s-9-1.34-9-3" }
            path { d: "M3 5v14c0 1.66 4 3 9 3s9-1.34 9-3V5" }
        }
    }
}

pub fn FilledArrowRight() -> Element {
    rsx! {
        svg {
            width: "16",
            height: "16",
            view_box: "0 0 24 24",
            fill: "currentColor",
            path { d: "M8 5v14l11-7z" }
        }
    }
}

pub fn FilledArrowUp() -> Element {
    rsx! {
        svg {
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "currentColor",
            path { d: "M7 14l5-5 5 5z" }
        }
    }
}

pub fn FilledArrowDown() -> Element {
    rsx! {
        svg {
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "currentColor",
            path { d: "M7 10l5 5 5-5z" }
        }
    }
}

#[component]
pub fn ArrowDown() -> Element {
    rsx! {
        svg {
            width: "20",
            height: "20",
            view_box: "0 0 24 24",
            fill: "currentColor",
            path { d: "M10 6 8.59 7.41 13.17 12l-4.58 4.59L10 18l6-6z" }
        }
    }
}

#[component]
pub fn ArrowRight() -> Element {
    rsx! {
        svg {
            width: "20",
            height: "20",
            view_box: "0 0 24 24",
            fill: "currentColor",
            path { d: "M7.41 8.59 12 13.17l4.59-4.58L18 10l-6 6-6-6 1.41-1.41z" }
        }
    }
}

#[component]
pub fn Clock() -> Element {
    rsx! {
        svg {
            "aria-hidden": "true",
            view_box: "0 0 24 24",
            width: "20px",
            height: "20px",
            display: "inline-block",
            fill: "currentColor",
            path { d: "M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8z" }
            path { d: "M12.5 7H11v6l5.25 3.15.75-1.23-4.5-2.67z" }
        }
    }
}

#[component]
pub fn Close() -> Element {
    rsx! {
        svg {
            "aria-hidden": "true",
            view_box: "0 0 24 24",
            width: "20px",
            height: "20px",
            display: "inline-block",
            fill: "currentColor",
            path { d: "M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z" }
        }
    }
}

#[component]
pub fn Discord() -> Element {
    rsx! {
        svg {
            width: "20px",
            height: "20px",
            fill: "#5F6060",
            view_box: "0 0 640 512",
            path {
                d: "M524.531 69.836a1.5 1.5 0 0 0-.764-.7A485.065 485.065 0 0 0 404.081 32.03a1.816 1.816 0 0 0-1.923.91 337.461 337.461 0 0 0-14.9 30.6 447.848 447.848 0 0 0-134.426 0 309.541 309.541 0 0 0-15.135-30.6 1.89 1.89 0 0 0-1.924-.91 483.689 483.689 0 0 0-119.688 37.107 1.712 1.712 0 0 0-.788.676C39.068 183.651 18.186 294.69 28.43 404.354a2.016 2.016 0 0 0 .765 1.375 487.666 487.666 0 0 0 146.825 74.189 1.9 1.9 0 0 0 2.063-.676A348.2 348.2 0 0 0 208.12 430.4a1.86 1.86 0 0 0-1.019-2.588 321.173 321.173 0 0 1-45.868-21.853 1.885 1.885 0 0 1-.185-3.126 251.047 251.047 0 0 0 9.109-7.137 1.819 1.819 0 0 1 1.9-.256c96.229 43.917 200.41 43.917 295.5 0a1.812 1.812 0 0 1 1.924.233 234.533 234.533 0 0 0 9.132 7.16 1.884 1.884 0 0 1-.162 3.126 301.407 301.407 0 0 1-45.89 21.83 1.875 1.875 0 0 0-1 2.611 391.055 391.055 0 0 0 30.014 48.815 1.864 1.864 0 0 0 2.063.7A486.048 486.048 0 0 0 610.7 405.729a1.882 1.882 0 0 0 .765-1.352c12.264-126.783-20.532-236.912-86.934-334.541ZM222.491 337.58c-28.972 0-52.844-26.587-52.844-59.239s23.409-59.241 52.844-59.241c29.665 0 53.306 26.82 52.843 59.239 0 32.654-23.41 59.241-52.843 59.241Zm195.38 0c-28.971 0-52.843-26.587-52.843-59.239s23.409-59.241 52.843-59.241c29.667 0 53.307 26.82 52.844 59.239 0 32.654-23.177 59.241-52.844 59.241Z",
            }
        }
    }
}

#[component]
pub fn GitHub() -> Element {
    rsx! {
        svg {
            width: "20px",
            height: "20px",
            view_box: "0 0 24 24",
            fill: "#5F6060",
            path { d: "M12 1.27a11 11 0 00-3.48 21.46c.55.09.73-.28.73-.55v-1.84c-3.03.64-3.67-1.46-3.67-1.46-.55-1.29-1.28-1.65-1.28-1.65-.92-.65.1-.65.1-.65 1.1 0 1.73 1.1 1.73 1.1.92 1.65 2.57 1.2 3.21.92a2 2 0 01.64-1.47c-2.47-.27-5.04-1.19-5.04-5.5 0-1.1.46-2.1 1.2-2.84a3.76 3.76 0 010-2.93s.91-.28 3.11 1.1c1.8-.49 3.7-.49 5.5 0 2.1-1.38 3.02-1.1 3.02-1.1a3.76 3.76 0 010 2.93c.83.74 1.2 1.74 1.2 2.94 0 4.21-2.57 5.13-5.04 5.4.45.37.82.92.82 2.02v3.03c0 .27.1.64.73.55A11 11 0 0012 1.27" }
        }
    }
}

#[component]
pub fn DateRange() -> Element {
    rsx! {
        svg {
            width: "20px",
            height: "20px",
            view_box: "0 0 24 24",
            fill: "currentColor",
            path { d: "M9 11H7v2h2v-2zm4 0h-2v2h2v-2zm4 0h-2v2h2v-2zm2-7h-1V2h-2v2H8V2H6v2H5c-1.11 0-1.99.9-1.99 2L3 20c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 16H5V9h14v11z" }
        }
    }
}

#[component]
pub fn ChevronRight() -> Element {
    rsx! {
        svg {
            width: "20px",
            height: "20px",
            view_box: "0 0 24 24",
            fill: "currentColor",
            path { d: "M10 6L8.59 7.41 13.17 12l-4.58 4.59L10 18l6-6z" }
        }
    }
}

#[component]
pub fn Calendar() -> Element {
    rsx! {
        svg {
            width: "20px",
            height: "20px",
            view_box: "0 0 24 24",
            fill: "currentColor",
            path { d: "M17 12h-5v5h5v-5zM16 1v2H8V1H6v2H5c-1.11 0-1.99.9-1.99 2L3 19c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2h-1V1h-2zm3 18H5V8h14v11z" }
        }
    }
}
