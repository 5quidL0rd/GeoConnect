use dioxus::prelude::*;

#[component]
pub fn Loading() -> Element {
    rsx! {
        div {
            class: "loading-screen",

            div {
                class: "loading-torii-wrap",

                div { class: "loading-glow" }
                div { class: "loading-portal-light" }

                svg {
                    class: "loading-torii",
                    view_box: "0 0 200 180",
                    xmlns: "http://www.w3.org/2000/svg",

                    rect { class: "torii-part torii-pillar", x: "40", y: "50", width: "14", height: "120" }
                    rect { class: "torii-part torii-pillar", x: "146", y: "50", width: "14", height: "120" }

                    path {
                        class: "torii-part torii-kasagi",
                        d: "M18 56 Q100 20 182 56 L182 70 Q100 37 18 70 Z",
                    }
                    rect { class: "torii-part", x: "30", y: "70", width: "140", height: "9" }

                    rect { class: "torii-part", x: "93", y: "79", width: "14", height: "31" }
                    rect { class: "torii-part", x: "40", y: "110", width: "120", height: "10" }
                }
            }

            p { class: "loading-text", "Crossing into Japan…" }
        }
    }
}
