use crate::server::auth::model::User;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ProfileDetailsProps {
    pub user: User,
    pub dark_mode: bool,
    pub user_token: String,
}

#[component]
pub fn ProfileDetails(props: ProfileDetailsProps) -> Element {
    let theme_class = if props.dark_mode {
        "bg-gray-800 text-white"
    } else {
        "bg-white text-gray-900"
    };

    rsx! {
        div { class: "p-6 rounded-lg {theme_class}",
            div { class: "flex items-center space-x-4 mb-6",
                img {
                    class: "w-20 h-20 rounded-full border-2 border-gray-300",
                    src: "{props.user.photo}",
                    alt: "User Photo"
                }
                div {
                    h2 { class: "text-2xl font-semibold", "{props.user.name}" }
                    p { class: "text-sm text-gray-500", "{props.user.email}" }
                }
            }

            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                ProfileItem { icon: "fa-id-badge", label: "User ID:", value: &props.user.id.to_string() }
                ProfileItem { icon: "fa-user-tag", label: "Role:", value: &props.user.role }
                ProfileItem { icon: "fa-check-circle", label: "Verified:", value: if props.user.verified { "Yes" } else { "No" } }
                ProfileItem { icon: "fa-id-card", label: "License Number:", value: props.user.license_number.as_deref().unwrap_or("N/A") }
                ProfileItem { icon: "fa-microchip", label: "ELD Device ID:", value: props.user.eld_device_id.as_deref().unwrap_or("N/A") }
                ProfileItem { icon: "fa-calendar-alt", label: "Registered At:", value: &props.user.created_at.format("%B %d, %Y").to_string() }
            }
        }
    }
}

#[component]
fn ProfileItem(icon: &'static str, label: &'static str, value: String) -> Element {
    rsx! {
        div { class: "flex items-center space-x-3 p-3 rounded-md",
            i { class: "fas {icon} text-gray-500 dark:text-gray-300 text-lg" }
            div { class: "flex justify-between w-full",
                span { class: "font-medium", "{label}" }
                span { class: "text-gray-600 dark:text-gray-300", "{value}" }
            }
        }
    }
}
