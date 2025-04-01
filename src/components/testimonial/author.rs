use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AuthorProps {
    author_name: &'static str,
    author_title: &'static str,
}

#[component]
pub fn AuthorInfo(props: AuthorProps) -> Element {
    rsx! {
        div { class: "flex items-center justify-center mt-4 space-x-4",
            div { class: "text-left",
                p { class: "text-sm font-semibold", "{props.author_name}" }
                p { class: "text-xs text-gray-500", "{props.author_title}" }
            }
        }
    }
}
