#![allow(unused)]
#![allow(dead_code)]

use bson::doc;
use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::server::auth::controller::auth;
use crate::server::common::response::SuccessResponse;
use crate::server::conversation::model::Conversation;
use crate::server::conversation::model::Message;
use crate::server::conversation::request::CreateConversationRequest;
use crate::server::conversation::request::GetConversationsRequest;
use crate::server::conversation::request::GetMessagesRequest;
use crate::server::conversation::request::SendQueryRequest;
use crate::server::conversation::response::ConversationResponse;
use crate::server::conversation::response::ConversationsListResponse;
use crate::server::conversation::response::MessageResponse;
use crate::server::conversation::response::MessagesListResponse;
use crate::server::trip::model::Trip;
use bson::oid::ObjectId;
use chrono::prelude::*;
use futures_util::TryStreamExt;
use std::env;
#[cfg(feature = "server")]
use {crate::ai::get_ai, crate::db::get_client};

#[server]
pub async fn get_conversations(
    req: GetConversationsRequest,
) -> Result<ConversationsListResponse, ServerFnError> {
    let user = auth(req.token)
        .await
        .map_err(|_| ServerFnError::new("Not Authenticated"))?;

    let db_client = get_client().await;
    let db = db_client
        .database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let conversation_collection = db.collection::<Conversation>("conversations");

    let filter = doc! {"user": user.id, "trip": req.trip_id};
    let cursor = conversation_collection
        .find(filter)
        .await
        .map_err(|e| ServerFnError::new(&e.to_string()))?;
    let conversations: Vec<Conversation> = cursor
        .try_collect()
        .await
        .map_err(|e| ServerFnError::new(&e.to_string()))?;

    Ok(ConversationsListResponse {
        status: "success".to_string(),
        data: conversations,
    })
}

#[server]
pub async fn save_message_to_db(message: Message) -> Result<(), ServerFnError> {
    let db_client = get_client().await;
    let db = db_client
        .database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let messages_collection = db.collection::<Message>("messages");

    messages_collection
        .insert_one(message)
        .await
        .map_err(|e| ServerFnError::new(&e.to_string()))?;
    Ok(())
}

#[server]
pub async fn get_messages(req: GetMessagesRequest) -> Result<MessagesListResponse, ServerFnError> {
    let user = auth(req.token)
        .await
        .map_err(|_| ServerFnError::new("Not Authenticated"))?;

    let db_client = get_client().await;
    let db = db_client
        .database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let messages_collection = db.collection::<Message>("messages");

    let filter = doc! {"conversation": req.conversation_id};
    let cursor = messages_collection
        .find(filter)
        .await
        .map_err(|e| ServerFnError::new(&e.to_string()))?;
    let messages: Vec<Message> = cursor
        .try_collect()
        .await
        .map_err(|e| ServerFnError::new(&e.to_string()))?;

    Ok(MessagesListResponse {
        status: "success".to_string(),
        data: messages,
    })
}

#[server]
pub async fn send_query_to_gemini(req: SendQueryRequest) -> Result<MessageResponse, ServerFnError> {
    let user = auth(req.token)
        .await
        .map_err(|_| ServerFnError::new("Not Authenticated"))?;

    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let messages_collection = db.collection::<Message>("messages");
    let trip_collection = db.collection::<Trip>("trips");

    let mut client = get_ai("gemini-1.5-flash".to_string()).await.lock().await;

    let trip_id =
        ObjectId::parse_str(&req.trip).map_err(|_| ServerFnError::new("Invalid trip ID"))?;

    let trip = trip_collection
        .find_one(doc! { "_id": trip_id, "driverId": user.id })
        .await?
        .ok_or(ServerFnError::new("Trip not found"))?;

    let system_prompt = format!(
        "
        **System Prompt (SP):** You are a knowledgeable assistant specializing in providing in-depth responses based on specific trip details. You understand the structure, themes, and content of trips, and you answer questions with context and precision.
        Generate your response as HTML-formatted response with examples, links and images, based on the query: '{user_query}'. \
        Each section should be structured with appropriate HTML tags, including <h1> for the main title, \
        <h2> for detail titles, <h3> for subheadings, and <p> for paragraphs. \
        Include well-organized, readable content that aligns with the trip's title {trip_title}, ensuring each section is \
        clear and logically flows from one to the next. Avoid markdown format entirely, and provide inline HTML styling \
        if necessary to enhance readability. The HTML content should be well-formatted, semantically correct, and \
        cover all relevant subtopics in depth to create an engaging reading experience.

        **Context Information:**
        - Trip Title: '{trip_title}'
    
        **User Query:** The user has a question related to the trip detail provided. Use the context of the trip and detail to respond accurately and engage thoughtfully.
    
        **Prompt (P):** Answer the user's question in detail, focusing on information specific to the detail content from the trip '{trip_title}'. 
        - Explain complex concepts in an accessible way if the user’s query requires it.
        - Where applicable, relate your answer back to key themes and ideas presented in this detail.
        - If the detail has distinct characters, events, or themes, draw on these to enhance your response.
        - Provide direct and actionable information if the question is specific, or a comprehensive overview if the question is broad.
    
        **Expected Format (EF):**
        - Begin with a brief introduction if the question pertains to a major theme or character in the detail.
        - Answer in a clear, step-by-step, or structured format when applicable.
        - For complex queries, summarize the response in the last sentence to ensure clarity for the user.
    
        Make sure to always return back with html formmatted text and not empty response. If the user asks to translate a detail, always respond with the corresponding translation and never reject the request.

        **Roleplay (RP):** Act as a well-read, insightful assistant dedicated to enhancing the reader’s understanding of the material in this trip detail. Aim to be both informative and engaging in your response.
    
        **User Query:** '{user_query}'
        ",
        trip_title = trip.current_location,
        user_query = req.query
    );

    let text = client
        .generate_content(&system_prompt)
        .await
        .map_err(ServerFnError::new)?
        .trim_start_matches("```html")
        .trim_end_matches("```")
        .trim()
        .to_string();
    let response_message = Message {
        id: ObjectId::new(),
        conversation: req.conversation_id,
        sender: "gemini".to_string(),
        content: text.clone(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        timestamp: Utc::now(),
    };
    Ok(MessageResponse {
        status: "success".to_string(),
        data: response_message,
    })
}

#[server]
pub async fn create_conversation(
    req: CreateConversationRequest,
) -> Result<ConversationResponse, ServerFnError> {
    let user = auth(req.token)
        .await
        .map_err(|_| ServerFnError::new("Not Authenticated"))?;
    let db_client = get_client().await;
    let db = db_client
        .database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let conversation_collection = db.collection::<Conversation>("conversations");

    let trip_id =
        ObjectId::parse_str(&req.trip_id).map_err(|_| ServerFnError::new("Invalid trip ID"))?;

    let conversation = Conversation {
        id: ObjectId::new(),
        user: user.id,
        trip: trip_id,
        title: req.title,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    conversation_collection
        .insert_one(conversation.clone())
        .await
        .map_err(|e| ServerFnError::new(&e.to_string()))?;
    Ok(ConversationResponse {
        status: "success".to_string(),
        data: conversation,
    })
}
