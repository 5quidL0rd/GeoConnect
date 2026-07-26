use dioxus::prelude::*;

#[component]
pub fn BackButton() -> Element {
    let nav = use_navigator();

    rsx! {
        button {
            class: "back-btn",
            onclick: move |_| {
                nav.go_back();
            },
            span { class: "back-btn-icon", "←" }
            "Back"
        }
    }
}
