use crate::theme::Theme;
use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    let dark_mode = use_context::<Signal<Theme>>();

    rsx! {
        section {
            id: "home",
            class: format!(
                "min-h-screen flex flex-col items-center justify-center transition-colors duration-300 px-6 {}",
                if dark_mode() == Theme::Dark { "bg-gray-900 text-white" } else { "bg-white text-black" }
            ),
            div {
                class: "text-center space-y-8",
                p {
                    class: "text-lg font-semibold uppercase tracking-widest text-transparent bg-clip-text bg-gradient-to-r from-blue-500 via-teal-500 to-green-500 animate-pulse",
                    "Effortless ELD Logging & AI Insights"
                }
                h1 {
                    class: "text-5xl md:text-7xl font-extrabold leading-tight animate-fade-in",
                    "Drive. Log. Optimize."
                },
                p {
                    class: "text-xl md:text-2xl text-gray-600 dark:text-gray-300 max-w-3xl mx-auto animate-fade-in delay-150",
                    "Automate your ELD logs, track trips, and get AI-powered summaries. All in one platform."
                },
                div {
                    class: "flex justify-center space-x-6 animate-slide-up delay-200",
                    a {
                        class: "bg-blue-500 text-white py-2 px-6 rounded-full shadow-lg hover:bg-blue-600 focus:outline-none transform hover:scale-105 transition-transform duration-150 font-semibold",
                        href: "/login",
                        "Get Started"
                    },
                }
            }
        }
    }
}
