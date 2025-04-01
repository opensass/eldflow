pub(crate) mod grid;
pub(crate) mod item;

use crate::components::common::header::Header;
use crate::components::features::grid::Grid;
use crate::theme::Theme;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
struct Feature {
    icon: &'static str,
    title: &'static str,
    description: &'static str,
}

#[component]
pub fn Features() -> Element {
    let dark_mode = use_context::<Signal<Theme>>();

    let features = vec![
        Feature {
            icon: "fas fa-clipboard-list text-blue-500 group-hover:animate-pulse",
            title: "ELD Log Management",
            description: "Seamlessly log, review, and manage daily ELD logs with a user-friendly interface.",
        },
        Feature {
            icon: "fas fa-route text-green-500 group-hover:animate-bounce",
            title: "Trip Tracking",
            description: "Track trips from start to finish, ensuring accurate logs of time and location.",
        },
        Feature {
            icon: "fas fa-brain text-indigo-500 group-hover:animate-spin",
            title: "AI-Powered Summaries",
            description: "Let AI analyze and summarize daily driver activity for quick insights and compliance.",
        },
        Feature {
            icon: "fas fa-shield-alt text-yellow-500 group-hover:animate-ping",
            title: "Compliance & Safety",
            description: "Stay compliant with automatic HOS tracking and smart alerts for rule violations.",
        },
        Feature {
            icon: "fas fa-sync-alt text-red-500 group-hover:animate-pulse",
            title: "Real-Time Sync",
            description: "Sync logs and trip details across devices with real-time cloud storage integration.",
        },
        Feature {
            icon: "fas fa-code text-purple-500 group-hover:animate-bounce",
            title: "Developer Friendly",
            description: "Built on Rust for high performance, offering flexible APIs for fleet management tools.",
        },
    ];

    rsx! {
        section {
            id: "features",
            class: format!("py-20 px-8 md:px-4 font-roboto flex min-h-screen justify-center transition-colors duration-300 {}",
                if dark_mode() == Theme::Dark { "bg-gray-900 text-white" } else { "bg-gray-100 text-gray-900" }),

            div { class: "max-w-5xl mx-auto text-center space-y-12",

                div { class: "relative mb-12 space-y-6",
                    Header {
                        title: "Why ELDFlow?",
                        subtitle: "Built on Rust and powered by AI, ELDFlow simplifies log tracking, trip timing, and compliance for professional drivers."
                    }
                }

                Grid { features: features }
            }
        }
    }
}
