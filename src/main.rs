use axum::{
    routing::get,
    Router,
    Json,
};
use serde::Serialize;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[derive(Serialize)]
struct ApiResponse {
    status: String,
    message: String,
    version: String,
    author: String,
}

async fn api_handler() -> Json<ApiResponse> {
    Json(ApiResponse {
        status: "success".into(),
        message: "Welcome to Sazumi Viki's API".into(),
        version: "1.0.0".into(),
        author: "Sazumi Viki".into(),
    })
}

async fn health_handler() -> Json<ApiResponse> {
    Json(ApiResponse {
        status: "healthy".into(),
        message: "All systems operational".into(),
        version: "1.0.0".into(),
        author: "Sazumi Viki".into(),
    })
}

async fn info_handler() -> Json<ApiResponse> {
    Json(ApiResponse {
        status: "success".into(),
        message: "nothing here yet".into(),
        version: "1.0.0".into(),
        author: "Sazumi Viki".into(),
    })
}

#[tokio::main]
async fn main() {
    let static_files_service = ServeDir::new("static");

    let app = Router::new()
        .route("/api", get(api_handler))
        .route("/api/health", get(health_handler))
        .route("/api/info", get(info_handler))
        .nest_service("/", static_files_service.clone())
        .fallback_service(static_files_service);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 Portfolio running at http://localhost:3000");
    
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app
    ).await.unwrap();
}