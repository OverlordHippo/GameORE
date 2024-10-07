use dioxus::prelude::*;

use crate::components::{Activity, Balance};

pub fn Home() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-16 grow overflow-visible",
            style: "min-height: 100vh;",
            Balance {}
            UnityGame {}
            Activity {}
        }
    }
}

pub fn UnityGame() -> Element {
    rsx! {
        div {
            class: "flex-grow relative",
            style: "width: 100%; max-width: 1920px; height: 0; padding-bottom: 56.25%;",
            iframe {
                class: "absolute top-0 left-0 w-full h-full",
                src: "/unity/index.html",
                width: "100%",
                height: "100%",
                allow: "autoplay; fullscreen; vr",
                allowfullscreen: true,
            }
        }
    }
}
