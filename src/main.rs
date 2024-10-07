use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async  {"the start"}));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:1111").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

