use crate::components::dashboard::fields::input::InputField;
use crate::components::dashboard::fields::number::NumberField;
use crate::components::spinner::Spinner;
use crate::components::spinner::SpinnerSize;
use crate::components::toast::manager::ToastManager;
use crate::components::toast::manager::ToastType;
use crate::server::trip::controller::fetch_google_places_autocomplete;
use crate::server::trip::controller::store_trip;
use crate::server::trip::request::StoreTripRequest;
use crate::theme::Theme;
use chrono::Duration;
use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use serde::Deserialize;

#[derive(Deserialize)]
struct GooglePlacesResponse {
    predictions: Vec<Prediction>,
}

#[derive(Deserialize)]
struct Prediction {
    description: String,
    place_id: String,
}

#[component]
pub fn CreateTripPanel(user_token: Signal<String>) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let dark_mode = theme() == Theme::Dark;
    let api_key = use_signal(|| "AIzaSyDsCnHPmTHj0Ijr82M53Ted5Qkx71CSDnw".to_string());

    // Trip Details
    let mut current_location = use_signal(|| "Beirut, Lebanon".to_string());
    let mut pickup_location = use_signal(|| "Jounieh, Lebanon".to_string());
    let mut dropoff_location = use_signal(|| "Tripoly, Lebanon".to_string());
    let mut cycle_used = use_signal(|| 1.0);

    let current_location_valid = use_signal(|| true);
    let pickup_location_valid = use_signal(|| true);
    let dropoff_location_valid = use_signal(|| true);

    let validate_location = |input: &str| !input.is_empty();

    let mut recommended_locations = use_signal(|| vec![]);
    let mut selected_route = use_signal(|| {
        Some(format!(
            "https://www.google.com/maps/embed/v1/directions?key={}&origin={}&destination={}",
            api_key(),
            current_location(),
            pickup_location()
        ))
    });
    let mut loading = use_signal(|| false);

    let mut toasts_manager = use_context::<Signal<ToastManager>>();

    let mut fetch_location_suggestions = move |input: String| {
        if input.is_empty() {
            recommended_locations.set(vec![]);
            return;
        }

        spawn(async move {
            match fetch_google_places_autocomplete(input.clone(), api_key()).await {
                Ok(response) => {
                    let suggestions: Vec<String> = response
                        .predictions
                        .iter()
                        .map(|p| p.description.clone())
                        .collect();
                    recommended_locations.set(suggestions);
                }
                Err(_) => {
                    recommended_locations.set(vec![]);
                }
            }
        });
    };

    let handle_location_input =
        move |e: Event<FormData>, signal: &mut Signal<String>, valid_signal: &mut Signal<bool>| {
            let input = e.value();
            signal.set(input.clone());
            valid_signal.set(validate_location(&input));
            fetch_location_suggestions(input);
        };

    let handle_submit = move |e: Event<FormData>| {
        e.stop_propagation();
        loading.set(true);

        if current_location().is_empty()
            || pickup_location().is_empty()
            || dropoff_location().is_empty()
        {
            toasts_manager.set(
                toasts_manager()
                    .add_toast(
                        "Error".into(),
                        "All trip details are required!".into(),
                        ToastType::Error,
                        Some(Duration::seconds(5)),
                    )
                    .clone(),
            );
            loading.set(false);
            return;
        }

        let route_url = format!(
            "https://www.google.com/maps/embed/v1/directions?key={}&origin={}&destination={}&waypoints={}",
            api_key(),
            pickup_location().replace(" ", "+"),
            dropoff_location().replace(" ", "+"),
            current_location().replace(" ", "+")
        );

        selected_route.set(Some(route_url));
        spawn(async move {
            let route_url = format!(
                "https://maps.googleapis.com/maps/api/distancematrix/json?origins={}&destinations={}&key={}",
                current_location().replace(" ", "+"),
                dropoff_location().replace(" ", "+"),
                api_key()
            );
            let store_request = StoreTripRequest {
                token: user_token(),
                current_location: current_location(),
                pickup_location: pickup_location(),
                dropoff_location: pickup_location(),
                cycle_used_hours: cycle_used(),
                status: "pending".to_string(),
                route_url: route_url.to_string(),
            };

            match store_trip(store_request).await {
                Ok(_) => {
                    toasts_manager.set(
                        toasts_manager()
                            .add_toast(
                                "Success".into(),
                                "Trip stored successfully!".into(),
                                ToastType::Success,
                                Some(Duration::seconds(5)),
                            )
                            .clone(),
                    );
                }
                Err(err) => {
                    toasts_manager.set(
                        toasts_manager()
                            .add_toast(
                                "Error".into(),
                                err.to_string(),
                                ToastType::Error,
                                Some(Duration::seconds(5)),
                            )
                            .clone(),
                    );
                }
            }
        });
        loading.set(false);
        // refresh cache
        LocalStorage::delete("trips_cache");
    };

    rsx! {
        div {
            class: format!("flex p-4 flex-col lg:flex-row {}",
                if dark_mode { "bg-gray-800 text-white" } else { "bg-white text-gray-900" }
            ),
            div {
                h2 { class: "text-xl font-semibold mb-4", "Plan A Trip" }
                form {
                    class: "space-y-4 flex-1",
                    onsubmit: handle_submit,

                    InputField { label: "Current Location", value: current_location, is_valid: current_location_valid, validate: validate_location, required: true }
                    InputField { label: "Pickup Location", value: pickup_location, is_valid: pickup_location_valid, validate: validate_location, required: true }
                    InputField { label: "Dropoff Location", value: dropoff_location, is_valid: dropoff_location_valid, validate: validate_location, required: true }
                    NumberField { label: "Current Cycle Used (Hrs)", value: cycle_used, required: true }

                    button {
                        class: format!("flex items-center space-x-2 bg-blue-500 text-white px-4 py-2 rounded {}", if dark_mode { "bg-blue-600" } else { "" }),
                        r#type: "submit",
                        disabled: loading(),
                        if loading() {
                            Spinner {
                                aria_label: "Loading spinner".to_string(),
                                size: SpinnerSize::Md,
                                dark_mode: true,
                            }
                            span { "Calculating Route..." }
                        } else {
                            span { "Store Trip" }
                        }
                    }
                }
            }

            if !recommended_locations().is_empty() {
                div {
                    class: "mt-2 bg-white border rounded shadow-lg p-2 absolute z-10 w-full",
                    for suggestion in recommended_locations() {
                        div {
                            class: "p-2 hover:bg-gray-200 cursor-pointer",
                            onclick: move |_| {
                                pickup_location.set(suggestion.clone());
                                recommended_locations.set(vec![]);
                            },
                            "{suggestion}"
                        }
                    }
                }
            }
                if let Some(route) = selected_route() {
                    div {
                        class: "mb-5 lg:mt-0 lg:ml-8 flex-1 w-full h-[800px] max-w-full",
                        h2 { class: "text-xl font-semibold mb-4", "Route Preview" }
                        div {
                            class: "w-full h-full",
                            iframe {
                                src: "{route}",
                                class: "w-full h-full border-0",
                                allowfullscreen: "true",
                            }
                        }
                    }
            }

        }
    }
}
