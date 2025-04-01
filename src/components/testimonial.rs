pub(crate) mod author;
pub(crate) mod card;
pub(crate) mod rating;

use crate::components::testimonial::author::AuthorInfo;
use crate::components::testimonial::rating::StarRating;
use crate::theme::Theme;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TestimonialData {
    quote: &'static str,
    author_name: &'static str,
    author_title: &'static str,
    star_images: Vec<&'static str>,
}

#[allow(unused_mut)]
#[component]
pub fn Testimonial() -> Element {
    let testimonials = vec![
        TestimonialData {
            quote: "I let ELDFlow handle my logs. Now my boss thinks I'm the most organized trucker alive. Jokes on him, I still lose my sunglasses daily.",
            author_name: "Mike Thompson",
            author_title: "Long-Haul Trucker",
            star_images: vec!["fas fa-star"; 5],
        },
        TestimonialData {
            quote: "I asked ELDFlow to optimize my route. Somehow, I avoided every traffic jam, got my coffee exactly when I needed it, and even found a perfect parking spot at a packed truck stop. I think it might be magic.",
            author_name: "Sarah Mitchell",
            author_title: "Logistics Coordinator",
            star_images: vec!["fas fa-star"; 5],
        },
        TestimonialData {
            quote: "ELDFlow told me to take a break. I ignored it. Five minutes later, my truck's radio started playing 'Take It Easy' by the Eagles. I get it, AI. I get it.",
            author_name: "James Carter",
            author_title: "Owner-Operator",
            star_images: vec!["fas fa-star"; 5],
        },
    ];

    let dark_mode = use_context::<Signal<Theme>>();
    let mut current_index = use_signal(|| 0);

    client! {
        let vec_len = testimonials.len();
        let mut eval = document::eval(
            r#"
            setInterval(() => {
                dioxus.send("");
            }, 5000)
            "#,
        );

        use_hook(|| {
            spawn(async move {
                loop {
                    let _ = eval.recv::<String>().await;
                    current_index.set((current_index() + 1) % vec_len);
                }
            })
        });
    }

    rsx! {
        section {
            id: "testimonial",
            class: format!("flex flex-col items-center justify-center min-h-screen p-8 {}",
            if dark_mode() == Theme::Dark { "bg-gray-900 text-white" } else { "bg-white text-black" }),

            div { class: "flex flex-col items-center mb-8",
                h2 { class: "text-4xl font-bold text-center",
                    "What Drivers Are Saying About ELDFlow"
                }

                p { class: format!("mt-2 text-lg {}", if dark_mode() == Theme::Dark { "text-gray-300" } else { "text-gray-700" }),
                    "ELDFlow: The AI co-driver you didn't know you needed."
                }
            }

            div { class: "flex flex-wrap justify-center items-center gap-8 p-4",
                for (i, testimonial) in testimonials.iter().enumerate() {
                    div { class: format!("transition-transform duration-500 transform {}, hover:scale-105 hover:shadow-xl",
                        if current_index() == i { "opacity-100 scale-100" } else { "opacity-50 scale-75 blur-sm" }),
                        div { class: format!("{} p-8 rounded-xl shadow-2xl text-center max-w-sm border",
                            if dark_mode() == Theme::Dark { "border-gray-700 bg-gray-800" } else { "bg-white border-gray-300" }),
                            StarRating { star_images: testimonial.star_images.clone() }
                            blockquote {
                                class: format!("text-lg font-semibold italic {}",
                                    if dark_mode() == Theme::Dark { "text-gray-400" } else { "text-gray-600" }
                                ),
                                "{testimonial.quote}"
                            }
                            AuthorInfo {
                                author_name: testimonial.author_name,
                                author_title: testimonial.author_title,
                            }
                        }
                    }
                }
            }

            div { class: "flex justify-center mt-6 space-x-2",
                for (i, _) in testimonials.iter().enumerate() {
                    div { class: format!("w-3 h-3 rounded-full {} transition-all duration-300",
                        if current_index() == i { "bg-gradient-to-r from-blue-400 to-indigo-500" } else { "bg-gray-400" })
                    }
                }
            }
        }
    }
}
