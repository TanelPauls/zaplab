use axum::{Json, Router, routing::get};
use serde::Serialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[derive(Serialize)]
struct WelcomeResponse {
    message: &'static str,
}

async fn welcome() -> Json<WelcomeResponse> {
    Json(WelcomeResponse { message: "Welcome from rust!" })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(welcome));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}