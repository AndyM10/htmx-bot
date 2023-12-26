use askama::Template;
use axum::{
    response::IntoResponse,
    routing::{get, post},
    Form, Router,
};
use ollama_rs::{
    generation::chat::{request::ChatMessageRequest, ChatMessage, ChatMessageResponse},
    Ollama,
};
use serde::Deserialize;

#[derive(Template)]
#[template(path = "index.html")]
struct HomeTemplate;

#[derive(Template)]
#[template(path = "msgs.html")]
struct MessagesTemplate {
    response: Vec<MessageResponse>,
}

#[derive(Deserialize)]
struct MessageResponse {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct UserMessage {
    message: String,
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(home))
        .route("/generate", post(generate));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

async fn home() -> impl IntoResponse {
    HomeTemplate
}

async fn generate(Form(form): Form<UserMessage>) -> impl IntoResponse {
    let ollama = Ollama::default();
    let mut messages: Vec<ChatMessage> = vec![];
    let user_message = ChatMessage::user(form.message);
    messages.push(user_message);

    let stream: ChatMessageResponse = ollama
        .send_chat_messages(ChatMessageRequest::new(
            "llama2:chat".to_string(),
            messages.clone(),
        ))
        .await
        .unwrap();

    let bot_message = stream.message.unwrap();
    messages.push(bot_message);

    let response: Vec<MessageResponse> = messages
        .iter()
        .map(|message| MessageResponse {
            role: format!("{:?}", message.role),
            content: message.content.clone(),
        })
        .collect();

    MessagesTemplate { response }
}
