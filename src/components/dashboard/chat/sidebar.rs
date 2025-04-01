use crate::server::conversation::controller::create_conversation;
use crate::server::conversation::controller::get_conversations;
use crate::server::conversation::model::Conversation;
use crate::server::conversation::request::CreateConversationRequest;
use crate::server::conversation::request::GetConversationsRequest;
use crate::theme::Theme;
use bson::oid::ObjectId;
use chrono::Utc;

use dioxus::prelude::*;

#[component]
pub fn ConversationsSidebar(
    conversations: Signal<Vec<Conversation>>,
    selected_conversation: Signal<ObjectId>,
    token: String,
    trip_id: String,
) -> Element {
    let token_clone = token.clone();
    let trip_id_clone = trip_id.clone();
    let theme = use_context::<Signal<Theme>>();
    let dark_mode = theme() == Theme::Dark;

    use_effect(move || {
        let token = token.clone();
        let trip_id = trip_id.clone();

        spawn(async move {
            if let Ok(conv_list) =
                get_conversations(GetConversationsRequest { token, trip_id }).await
            {
                conversations.set(conv_list.data);
            }
        });
    });

    rsx! {
        div {
            class: format!("p-4 {}",
            if dark_mode { "bg-gray-900 text-white" } else { "bg-white text-gray-900" }),

            h3 { class: "text-lg font-semibold mb-4 text-blue-500", "Conversations" }

            button {
                class: "w-full bg-blue-500 text-white p-2 rounded-lg mb-4",
                onclick: move |_| {
                    let token_clone = token_clone.clone();
                    let trip_id_clone = trip_id_clone.clone();
                    spawn(async move {
                        let title = format!("Conversation {}", Utc::now().timestamp());
                        if let Ok(new_conversation) = create_conversation(CreateConversationRequest { trip_id: trip_id_clone, token: token_clone, title }).await {
                            let mut current_conversations = conversations();
                            current_conversations.push(new_conversation.data);
                            conversations.set(current_conversations);
                        }
                    });
                },
                "+ New Chat"
            }

            ul {
                for conversation in conversations().into_iter() {
                    li {
                        class: format!("p-2 cursor-pointer rounded-lg {}",
                            if conversation.id == selected_conversation() {
                                "bg-blue-200 text-black dark:bg-blue-600 font-bold"
                            } else {
                                "hover:bg-blue-100 hover:text-black dark:hover:bg-blue-800"
                            }
                        ),
                        onclick: move |_| selected_conversation.set(conversation.id),
                        "{conversation.title}"
                    }
                }
            }
        }
    }
}
