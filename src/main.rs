#![allow(unused)] // For beginning only.

use axum::extract::Json;
use axum::response::IntoResponse;
use axum::{
    http::StatusCode,
    routing::{get, get_service},
    Router,
};
use serde_json::json;
use std::net::SocketAddr;
use tower_http::services::{ServeDir, ServeFile};

async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Building a simple CRUD API with Rust, SQLX, Postgres,and Axum";

    let json_response = json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .nest_service(
            "/assets",
            get_service(ServeDir::new("./frontend/dist/assets")).handle_error(|_| async move {
                (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong…")
            }),
        )
        .fallback_service(
            get_service(ServeFile::new("./frontend/dist/index.html")).handle_error(
                |_| async move { (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error") },
            ),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("->> LISTENING on {addr}\n");

    println!("🚀 Server started successfully!!\n");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
