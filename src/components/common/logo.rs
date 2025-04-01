use dioxus::prelude::*;

#[component]
pub fn Logo() -> Element {
    rsx! {
        div { class: "flex items-center",
            img {
                src: asset!("/assets/logo.webp"),
                alt: "AI Trip Logo",
                class: "w-24 h-24 object-contain"
            }
        }
    }
}
