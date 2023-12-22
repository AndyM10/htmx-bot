use askama::Template;
use axum::{response::IntoResponse, routing::get, Router};

#[derive(Template)]
#[template(path = "index.html")]
struct HomeTemplate;

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(home));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

async fn home() -> impl IntoResponse {
    HomeTemplate
}
