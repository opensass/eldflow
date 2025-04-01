use crate::theme::Theme;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ItemProps {
    icon: &'static str,
    title: &'static str,
    description: &'static str,
}

#[component]
pub fn FeatureItem(props: ItemProps) -> Element {
    let dark_mode = use_context::<Signal<Theme>>();

    rsx! {
        div {
            class: format!(
                "flex flex-col items-center p-6 rounded-lg transition-all duration-300 border border-gray-300 hover:shadow-lg
                shadow-md {} {}",
                if dark_mode() == Theme::Dark { "bg-gray-800 hover:bg-gray-700" } else { "bg-white hover:bg-gray-100" },
                "transform hover:-translate-y-1 hover:shadow-lg"
            ),
            i {
                class: format!("text-3xl mb-4 transform transition-transform duration-300 hover:scale-110 {}", props.icon),
            }
            h3 {
                class: format!(
                    "text-lg font-semibold transition-colors duration-300 {}",
                    if dark_mode() == Theme::Dark { "text-white" } else { "text-gray-800" }
                ),
                "{props.title}"
            }
            p {
                class: format!(
                    "text-sm text-center transition-colors duration-300 {}",
                    if dark_mode() == Theme::Dark { "text-gray-400" } else { "text-gray-600" }
                ),
                "{props.description}"
            }
        }
    }
}
