use axum::{
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

// ====== Request Data ======
#[derive(Debug, Deserialize)]
struct RegisterRequest {
    first_name: String,
    last_name: String,
    email: String,
    password: String,
}

// ====== Response Data ======
#[derive(Debug, Serialize)]
struct RegisterResponse {
    success: bool,
    message: String,
}

#[tokio::main]
async fn main() {
    // Build router
    let app = Router::new()
        .route("/register", post(register_handler));

    // Server address
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("🚀 Server running on http://{}", addr);

    // Start server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// ====== Handler for POST /register ======
async fn register_handler(
    Json(payload): Json<RegisterRequest>
) -> Json<RegisterResponse> {
    println!("New user registered: {:?}", payload);

    Json(RegisterResponse {
        success: true,
        message: format!("User '{}' registered successfully!", payload.username),
    })
}
