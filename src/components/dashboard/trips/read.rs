use crate::components::spinner::{Spinner, SpinnerSize};
use crate::components::toast::manager::ToastManager;
use crate::components::toast::manager::ToastType;
use crate::server::trip::controller::get_elds_for_user;
use crate::server::trip::controller::get_trips_for_user;
use crate::server::trip::controller::store_eld_log;
use crate::server::trip::request::GetEldsForUserRequest;
use crate::server::trip::request::GetTripsForUserRequest;
use crate::server::trip::request::StoreEldLogRequest;
use crate::theme::Theme;
use chrono::Duration;
use chrono::Utc;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use eld::dioxus::Chart;
use eld::{clear_chart, DutyStatus, Segment};

#[component]
pub fn EldLogsPanel(trip_id: String, token: Signal<String>) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let dark_mode = theme() == Theme::Dark;
    let mut start_hour = use_signal(|| "".to_string());
    let mut end_hour = use_signal(|| "".to_string());
    let mut status = use_signal(|| DutyStatus::OffDuty);
    let mut location = use_signal(|| "".to_string());
    let mut note = use_signal(|| "".to_string());
    let theme = use_context::<Signal<Theme>>();
    let mut toasts_manager = use_context::<Signal<ToastManager>>();

    let mut eld_data = use_signal::<Vec<Segment>>(Vec::new);

    let mut selected_trip = use_signal(|| trip_id.clone());
    let mut trips = use_signal(Vec::new);
    let mut loading = use_signal(|| true);

    let _ = use_resource(move || async move {
        clear_chart();
        match get_trips_for_user(GetTripsForUserRequest { token: token() }).await {
            Ok(response) => {
                loading.set(false);
                trips.set(response.data.clone());
                selected_trip.set(response.data.clone()[0].id.to_string());
                match get_elds_for_user(GetEldsForUserRequest {
                    token: token(),
                    trip_id: response.data.clone()[0].id.to_string(),
                })
                .await
                {
                    Ok(response) => {
                        let mut new_segments = Vec::new();

                        for eld in response.data {
                            match (*eld.status).into() {
                                "OffDuty" => {
                                    new_segments.push(Segment {
                                        start_hour: eld.start_hour as f32,
                                        end_hour: eld.end_hour as f32,
                                        status: DutyStatus::OffDuty,
                                        location: eld.location.clone(),
                                        note: eld.note.clone(),
                                    });
                                }
                                "Sleeper" => {
                                    new_segments.push(Segment {
                                        start_hour: eld.start_hour as f32,
                                        end_hour: eld.end_hour as f32,
                                        status: DutyStatus::Sleeper,
                                        location: eld.location.clone(),
                                        note: eld.note.clone(),
                                    });
                                }
                                "Driving" => {
                                    new_segments.push(Segment {
                                        start_hour: eld.start_hour as f32,
                                        end_hour: eld.end_hour as f32,
                                        status: DutyStatus::Driving,
                                        location: eld.location.clone(),
                                        note: eld.note.clone(),
                                    });
                                }
                                "OnDuty" => {
                                    new_segments.push(Segment {
                                        start_hour: eld.start_hour as f32,
                                        end_hour: eld.end_hour as f32,
                                        status: DutyStatus::OnDuty,
                                        location: eld.location.clone(),
                                        note: eld.note.clone(),
                                    });
                                }
                                _ => (),
                            }
                        }

                        eld_data.set(new_segments);
                    }
                    Err(_) => {
                        eld_data.set(Vec::new());
                    }
                }
            }
            Err(_) => {
                loading.set(false);
            }
        }
    });
    let fetch_eld_data = move |trip_id: String| {
        clear_chart();
        spawn(async move {
            match get_elds_for_user(GetEldsForUserRequest {
                token: token(),
                trip_id: trip_id.clone(),
            })
            .await
            {
                Ok(response) => {
                    let mut new_segments = Vec::new();

                    for eld in response.data {
                        match (*eld.status).into() {
                            "OffDuty" => {
                                new_segments.push(Segment {
                                    start_hour: eld.start_hour as f32,
                                    end_hour: eld.end_hour as f32,
                                    status: DutyStatus::OffDuty,
                                    location: eld.location.clone(),
                                    note: eld.note.clone(),
                                });
                            }
                            "Sleeper" => {
                                new_segments.push(Segment {
                                    start_hour: eld.start_hour as f32,
                                    end_hour: eld.end_hour as f32,
                                    status: DutyStatus::Sleeper,
                                    location: eld.location.clone(),
                                    note: eld.note.clone(),
                                });
                            }
                            "Driving" => {
                                new_segments.push(Segment {
                                    start_hour: eld.start_hour as f32,
                                    end_hour: eld.end_hour as f32,
                                    status: DutyStatus::Driving,
                                    location: eld.location.clone(),
                                    note: eld.note.clone(),
                                });
                            }
                            "OnDuty" => {
                                new_segments.push(Segment {
                                    start_hour: eld.start_hour as f32,
                                    end_hour: eld.end_hour as f32,
                                    status: DutyStatus::OnDuty,
                                    location: eld.location.clone(),
                                    note: eld.note.clone(),
                                });
                            }
                            _ => (),
                        }
                    }

                    eld_data.set(new_segments);
                }
                Err(_) => {
                    eld_data.set(Vec::new());
                }
            }
        });
    };

    let add_log_entry = move |_| {
        let start = start_hour().parse::<f32>();
        let end = end_hour().parse::<f32>();

        if let (Ok(start), Ok(end)) = (start, end) {
            if start >= end || start < 0.0 || end > 24.0 {
                toasts_manager.set(
                    toasts_manager()
                        .add_toast(
                            "Error".into(),
                            "Invalid time range!".into(),
                            ToastType::Error,
                            Some(Duration::seconds(5)),
                        )
                        .clone(),
                );
                return;
            }

            eld_data.write().push(Segment {
                start_hour: start,
                end_hour: end,
                status: status(),
                location: location(),
                note: note(),
            });
            let trip_id = selected_trip();

            spawn(async move {
                let total_hours = eld_data.read().iter().fold([0.0; 4], |mut acc, segment| {
                    let duration = segment.end_hour - segment.start_hour;
                    match segment.status {
                        DutyStatus::OffDuty => acc[0] += duration,
                        DutyStatus::Sleeper => acc[1] += duration,
                        DutyStatus::Driving => acc[2] += duration,
                        DutyStatus::OnDuty => acc[3] += duration,
                        DutyStatus::PersonalConveyance | DutyStatus::YardMove => (),
                    }
                    acc
                });

                let store_request = StoreEldLogRequest {
                    token: token(),
                    trip_id: trip_id,
                    status: status().to_string(),
                    note: note(),
                    start_hour: start as f64,
                    end_hour: end as f64,
                    off_duty_hours: total_hours[0] as f64,
                    sleeper_berth_hours: total_hours[1] as f64,
                    driving_hours: total_hours[2] as f64,
                    on_duty_hours: total_hours[3] as f64,
                    location: location(),
                    odometer_reading: Some(0.0),
                };

                match store_eld_log(store_request).await {
                    Ok(_) => {
                        toasts_manager.set(
                            toasts_manager()
                                .add_toast(
                                    "Success".into(),
                                    "Log added successfully!".into(),
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
            start_hour.set("".to_string());
            end_hour.set("".to_string());
            location.set("".to_string());
            note.set("".to_string());
        } else {
            toasts_manager.set(
                toasts_manager()
                    .add_toast(
                        "Error".into(),
                        "Invalid numeric values.".into(),
                        ToastType::Error,
                        Some(Duration::seconds(5)),
                    )
                    .clone(),
            );
        }
    };

    let button_class = |s: DutyStatus| {
        let base = "status-button";
        let selected = if status() == s { "selected" } else { "" };
        format!(
            "{base} {selected} {}",
            match s {
                DutyStatus::OffDuty => "off-duty",
                DutyStatus::Sleeper => "sleeper",
                DutyStatus::OnDuty => "on-duty",
                DutyStatus::Driving => "driving",
                DutyStatus::PersonalConveyance => "pc",
                DutyStatus::YardMove => "ym",
            }
        )
    };

    let total_hours = eld_data().iter().fold([0.0; 4], |mut acc, segment| {
        let duration = segment.end_hour - segment.start_hour;
        match segment.status {
            DutyStatus::OffDuty => acc[0] += duration,
            DutyStatus::Sleeper => acc[1] += duration,
            DutyStatus::Driving => acc[2] += duration,
            DutyStatus::OnDuty => acc[3] += duration,
            DutyStatus::PersonalConveyance | DutyStatus::YardMove => (),
        }
        acc
    });

    rsx! {
            div {
            div {
                class: "grid grid-cols-1 lg:grid-cols-2 gap-0 p-2 items-start justify-start",
                form {
                    class: "p-4 pt-0 max-w-lg rounded-lg",
                    onsubmit: add_log_entry,

                    h2 { class: "text-2xl font-bold mb-4", "Enter Log Details" }
                    div { class: "mb-4",
                        label { class: "font-medium", "Select Trip: " }
                        select {
                            class: format!(
                                "mt-1 block w-full p-2 border rounded-md shadow-sm {}",
                                if dark_mode { "bg-gray-900 text-white" } else { "bg-white text-black" },
                            ),
                            onchange: move |e| {
                                let new_trip_id = e.value();
                                selected_trip.set(new_trip_id.clone());
                                fetch_eld_data(new_trip_id);
                            },
                            for trip in trips() {
                                option {
                                    value: "{trip.id}",
                                    selected: trip.id.to_string() == selected_trip(),
                                    "{trip.current_location} → {trip.dropoff_location}"
                                }
                            }
                        }
                    }
                    div { class: "flex flex-col md:flex-row justify-between gap-6",
                        div { class: "flex-1 flex flex-col",
                            label { class: "font-medium", "Start Hour: " }
                            input {
                                class: format!(
                                    "mt-1 block w-full p-2 border rounded-md shadow-sm {}",
                                    if dark_mode { "bg-gray-900" } else { "" }

                                ),
                                r#type: "number", value: "{start_hour}",
                                oninput: move |e| start_hour.set(e.value()),
                                min: 0, max: 24, step: 0.5, placeholder: "0 - 24", required: true
                            }
                        }
                        div { class: "flex-1 flex flex-col",
                            label { class: "font-medium", "End Hour: " }
                            input {
                                class: format!(
                                    "mt-1 block w-full p-2 border rounded-md shadow-sm {}",
                                    if dark_mode { "bg-gray-900" } else { "" },
                                ),
                                r#type: "number", value: "{end_hour}",
                                oninput: move |e| end_hour.set(e.value()),
                                min: 0, max: 24, step: 0.5, placeholder: "0 - 24", required: true
                            }
                        }
                    }

                    div { class: "status-grid",
                        button { r#type: "button", class: "{button_class(DutyStatus::OffDuty)}", onclick: move |_| status.set(DutyStatus::OffDuty),
                            div { class: "status-box", "OFF" }
                            span { class: "status-label", "Off Duty" }
                        }
                        button { r#type: "button", class: "{button_class(DutyStatus::Sleeper)}", onclick: move |_| status.set(DutyStatus::Sleeper),
                            div { class: "status-box", "SB" }
                            span { class: "status-label", "Sleeper Berth" }
                        }
                        button { r#type: "button", class: "{button_class(DutyStatus::OnDuty)}", onclick: move |_| status.set(DutyStatus::OnDuty),
                            div { class: "status-box", "ON" }
                            span { class: "status-label", "On Duty" }
                        }
                        button { r#type: "button", class: "{button_class(DutyStatus::Driving)}", onclick: move |_| status.set(DutyStatus::Driving),
                            div { class: "status-box", "DR" }
                            span { class: "status-label", "Driving" }
                        }
                    }

                    div { class: "mt-4",
                        label { class: "font-medium", "Location: " }
                        input {
                            class: format!(
                                "mt-1 block w-full p-2 border rounded-md shadow-sm {}",
                                if dark_mode { "bg-gray-900" } else { "" },
                            ),
                            value: "{location}",
                            oninput: move |e| location.set(e.value()),
                            placeholder: "Enter city or state", required: true
                        }
                    }

                    div { class: "mt-4",
                        label { class: "font-medium", "Note: " }
                        textarea {
                            class: format!(
                                "mt-1 block w-full p-2 border rounded-md shadow-sm {}",
                                if dark_mode { "bg-gray-900" } else { "" },
                            ),
                            value: "{note}",
                            oninput: move |e| note.set(e.value()),
                            placeholder: "Add a note...", required: true
                        }
                    }

                    button {
                        class: "w-full mt-4 p-3 bg-blue-600 text-white font-semibold rounded-md transition hover:bg-blue-700",
                        "Add Log"
                    }
                }

                div {
                    class: "rounded-lg",
                    h2 { class: "text-2xl font-bold mb-4", "ELD Chart" }

                    Chart {
                        data: eld_data,
                        width: 1000,
                        height: 450,
                        background_color: "#F0F0F0",
                        grid_color: "#BBBBBB",
                        font: "16px Arial",
                        label_color: "#222222",
                        on_duty_color: "#FFD700",
                    }
                }
            }

            div { class: "stats-container",
            h2 { class: "text-left text-2xl font-bold mb-4", "Log Summary" }
            table { class: "stats-table",
                thead {
                    tr {
                        th { "Status" }
                        th { "Hours" }
                    }
                }
                tbody {
                    tr { class: "off-duty",
                        td { "Off Duty" }
                        td { "{total_hours[0]:.2} hrs" }
                    }
                    tr { class: "sleeper",
                        td { "Sleeper Berth" }
                        td { "{total_hours[1]:.2} hrs" }
                    }
                    tr { class: "driving",
                        td { "Driving" }
                        td { "{total_hours[2]:.2} hrs" }
                    }
                    tr { class: "on-duty",
                        td { "On Duty" }
                        td { "{total_hours[3]:.2} hrs" }
                    }
                }
            }
        }
    }
        }
}
