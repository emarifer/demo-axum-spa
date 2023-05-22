// #![allow(unused)] // For beginning only.

use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service},
    Router,
};
use serde_json::json;
use std::net::SocketAddr;
use tower_http::services::{ServeDir, ServeFile};

async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Building a Full-stack SPA demo using Solidjs, TypeScript, and Tailwindcss. Backend by Rust/Axum.";

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

/*
 * CHECKEANDO DEL SERVIDOR CON EL COMANDO cURL. VER:
 * curl -v https://axum-spa.onrender.com/api/healthchecker | json_pp
 * (NO ES NECESARIO EL CORS SI EL SERVICIO DE LOS ASSETS ESTÁTICOS SE HACE DESDE EL MISMO SERVER)
 *
 * SIRVIENDO ESTÁTICOS DESDE AXUM. VER:
 * https://github.com/search?q=axum+blog&type=repositories
 * https://github.com/Ericarthurc/ericarthurc.com_axum_solid_OLD
 * https://github.com/Ericarthurc/ericarthurc.com_axum_OLD
 *
 * https://github.com/search?q=axum+solidjs&type=repositories
 * https://github.com/robertwayne/template-axum-solidjs-spa
 *
 * https://github.com/search?q=axum%20react&type=repositories
 * https://github.com/robertwayne/template-axum-react-spa
 *
 * https://github.com/search?q=axum%20yew&type=repositories
 * https://github.com/rksm/axum-yew-setup
 * https://robert.kra.hn/posts/2022-04-03_rust-web-wasm/
 * https://www.udemy.com/course/learn-full-stack-rust-programming-using-axum-yew-and-sqlx/
 * https://github.com/infinityfish/fullstackrustcourse
 *
 * https://www.google.com/search?q=axum+server+frontend&oq=axu&aqs=chrome.0.69i59l2j69i57j69i59j46i67i340i650j69i60l3.1955j0j4&sourceid=chrome&ie=UTF-8
 *
 * HTTP Cache Headers - A Complete Guide. VER:
 * https://www.keycdn.com/blog/http-cache-headers#:~:text=downloaded%20every%20time.-,max%2Dage,for%20the%20next%2090%20seconds.
 */
