pub(crate) mod panel;
pub(crate) mod sidebar;

use crate::components::dashboard::chat::panel::ChatPanel;
use crate::components::dashboard::chat::sidebar::ConversationsSidebar;
use crate::components::spinner::{Spinner, SpinnerSize};
use crate::server::conversation::controller::get_conversations;
use crate::server::conversation::model::Conversation;
use crate::server::conversation::request::GetConversationsRequest;
use crate::server::trip::controller::get_trips_for_user;
use crate::server::trip::request::GetTripsForUserRequest;
use crate::theme::Theme;
use bson::oid::ObjectId;
use chrono::Utc;
use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CachedConversationsData {
    pub trip_id: String,
    pub conversations: Vec<Conversation>,
    pub timestamp: i64,
}

pub const CONVERSATIONS_CACHE_KEY: &str = "conversations_cache";
pub const CONVERSATIONS_CACHE_TIMEOUT: i64 = 2 * 60 * 60;

#[component]
pub fn ChatPanelPage(user_token: Signal<String>, trip_id: String) -> Element {
    let selected_conversation = use_signal(ObjectId::new);
    let theme = use_context::<Signal<Theme>>();
    let dark_mode = theme() == Theme::Dark;
    let mut conversations = use_signal(Vec::<Conversation>::new);
    let mut loading = use_signal(|| true);
    let mut is_sidebar_visible = use_signal(|| false);
    let mut selected_trip = use_signal(|| trip_id.clone());
    let mut trips = use_signal(Vec::new);

    let _ = use_resource(move || async move {
        match get_trips_for_user(GetTripsForUserRequest {
            token: user_token(),
        })
        .await
        {
            Ok(response) => {
                loading.set(false);
                trips.set(response.data.clone());
                selected_trip.set(response.data.clone()[0].id.to_string());
            }
            Err(_) => {
                loading.set(false);
            }
        }
    });
    use_effect(move || {
        let trip_id = trip_id.clone();
        spawn(async move {
            let now = Utc::now().timestamp();

            if let Ok(cached_data) =
                LocalStorage::get::<CachedConversationsData>(CONVERSATIONS_CACHE_KEY)
            {
                if cached_data.trip_id == trip_id
                    && now - cached_data.timestamp < CONVERSATIONS_CACHE_TIMEOUT
                {
                    loading.set(false);
                    conversations.set(cached_data.conversations.clone());
                    return;
                }
            }

            if let Ok(response) = get_conversations(GetConversationsRequest {
                token: user_token(),
                trip_id: trip_id.clone(),
            })
            .await
            {
                loading.set(false);
                conversations.set(response.data.clone());

                let cached_data = CachedConversationsData {
                    trip_id: trip_id,
                    conversations: response.data.clone(),
                    timestamp: now,
                };
                let _ = LocalStorage::set(CONVERSATIONS_CACHE_KEY, &cached_data);
            } else {
                loading.set(true);
            }
        });
    });

    rsx! {
        div {
            class: "flex min-h-screen",
            div {
                class: format!("border-r border-gray-600 min-h-screen hidden md:block {}",
                if dark_mode { "bg-gray-900 text-white" } else { "bg-white text-gray-900" }),
                if loading() {
                    Spinner {
                        aria_label: "Loading conversations...".to_string(),
                        size: SpinnerSize::Md,
                        dark_mode: true,
                    }
                } else {
                    ConversationsSidebar {
                        conversations,
                        selected_conversation,
                        token: user_token(),
                        trip_id: selected_trip(),
                    }
                }
            }

            div {
                class: "flex-1 flex flex-col h-full dark:bg-gray-800",
                ChatPanel {
                    conversation_id: selected_conversation,
                    user_token: user_token,
                }
            }
            button {
                class: "fixed bottom-6 right-4 bg-black bg-opacity-50 z-10 md:hidden",
                onclick: move |_| {
                    is_sidebar_visible.set(!is_sidebar_visible());
                },
                "☰"
            }

            if is_sidebar_visible() {
                div {
                    class: format!("fixed left-0 top-0 w-3/4 z-20 shadow-lg min-h-screen md:hidden {}",
                    if dark_mode { "bg-gray-900 text-white" } else { "bg-white text-gray-900" }),
                    ConversationsSidebar {
                        conversations: conversations.clone(),
                        selected_conversation: selected_conversation.clone(),
                        token: user_token(),
                        trip_id: selected_trip(),
                    }
                }
            }
        }
    }
}
