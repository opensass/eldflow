use crate::components::spinner::{Spinner, SpinnerSize};
use crate::server::trip::controller::get_trips_for_user;
use crate::server::trip::model::Trip;
use crate::server::trip::request::GetTripsForUserRequest;
use crate::theme::Theme;
use chrono::Utc;
use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CachedTripsData {
    pub data: Vec<Trip>,
    pub timestamp: i64,
}

pub const CACHE_KEY: &str = "trips_cache";
pub const CACHE_TIMEOUT: i64 = 2 * 60 * 60;

#[component]
pub fn TripsPanel(user_token: Signal<String>) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let dark_mode = theme() == Theme::Dark;
    let mut trips = use_signal(Vec::new);
    let mut displayed_trips = use_signal(Vec::new);
    let mut loading = use_signal(|| true);
    let mut search_query = use_signal(String::new);

    let _ = use_resource(move || async move {
        let now = Utc::now().timestamp();

        if let Ok(cached_data) = LocalStorage::get::<CachedTripsData>(CACHE_KEY) {
            if now - cached_data.timestamp < CACHE_TIMEOUT {
                loading.set(false);
                trips.set(cached_data.data.clone());
                displayed_trips.set(cached_data.data);
                return;
            }
        }

        match get_trips_for_user(GetTripsForUserRequest {
            token: user_token(),
        })
        .await
        {
            Ok(response) => {
                let cached_data = CachedTripsData {
                    data: response.data.clone(),
                    timestamp: now,
                };
                let _ = LocalStorage::set(CACHE_KEY, &cached_data);

                loading.set(false);
                trips.set(response.data.clone());
                displayed_trips.set(response.data);
            }
            Err(_) => {
                loading.set(false);
            }
        }
    });

    // Function to filter trips based on search query
    let mut filter_trips = move || {
        let query = search_query().to_lowercase();

        let filtered_trips = trips()
            .iter()
            .filter(|trip| {
                let title_matches = trip.current_location.to_lowercase().contains(&query);
                let pickup_matches = trip.pickup_location.to_lowercase().contains(&query);
                let dropoff_matches = trip.dropoff_location.to_lowercase().contains(&query);

                title_matches || pickup_matches || dropoff_matches
            })
            .cloned()
            .collect::<Vec<_>>();

        displayed_trips.set(filtered_trips);
    };

    rsx! {
        div {
            div {
                class: "w-full md:w-1/3 pb-4 mb-4 md:mb-0 flex flex-col gap-8",

                div {
                    h3 { class: "text-2xl font-bold mb-4", "Search" }
                    input {
                        class: format!(
                            "mt-1 block w-full p-2 border rounded-md shadow-sm {}",
                            if dark_mode { "bg-gray-900 text-white" } else { "bg-white text-black" },
                        ),
                        placeholder: "Search by location...",
                        value: "{search_query()}",
                        oninput: move |e| {
                            search_query.set(e.value());
                            filter_trips();
                        },
                    }
                }
            }
            h2 { class: "text-xl font-semibold mb-4", "All Trips" }

            if displayed_trips.len() > 0 {
                div {
                    class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6",
                    for trip in displayed_trips() {
                        div {
                            class: format!(
                                "p-4 pl-0 shadow rounded-lg flex space-x-4 {}",
                                if dark_mode { "bg-gray-700 text-white" } else { "bg-gray-100 text-black" }
                            ),
                            div {
                                class: "pl-2 mt-1.5 flex flex-col",
                                div {
                                    class: "w-3 h-3 rounded-full border-2 border-gray-500",
                                }

                                div {
                                    class: "ml-1.5 h-4 border-l-2 border-gray-400",
                                }

                                 div {
                                    class: "w-3 h-3 bg-red-500 rounded-full",
                                }
                            }
                            div {
                                class: "flex flex-col",
                                p { class: "text-[16px] font-semibold", "{trip.current_location}" }
                                div { class: "h-1" }
                                p { class: "text-[16px] font-semibold text-red-500", "{trip.dropoff_location}" }
                                img {
                                    src: trip.picture,
                                    alt: "Trip cover",
                                    class: "w-full h-48 object-cover rounded-md my-2"
                                }
                                p { class: "text-sm", "Pickup: {trip.pickup_location}" }
                                p { class: "text-sm", "Status: {trip.status}" }
                                p { class: "text-sm", "Cycle Used Hours: {trip.cycle_used_hours}" }
                                p { class: "text-sm", {format!("Distance: {:.2} miles", trip.distance_miles.unwrap_or(0.0))} }
                                p {
                                    class: "text-sm",
                                    "Created: {trip.created_at.format(\"%B %d, %Y\")}"
                                }
                            }
                        }
                    }
                }
            } else {
                p {
                    class: "flex items-center space-x-2 px-4 py-2 rounded",
                    if loading() {
                        Spinner {
                            aria_label: "Loading spinner".to_string(),
                            size: SpinnerSize::Md,
                            dark_mode: dark_mode,
                        }
                        span { "Loading trips..." }
                    } else {
                        span { "No trips found." }
                    }
                }
            }
        }
    }
}
