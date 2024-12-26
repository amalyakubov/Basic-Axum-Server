mod handlers;
mod models;
mod state;

use axum::{
    routing::{get, post},
    Router,
};
use handlers::{add_user, get_user, get_users};
use state::AppState;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let app_state = AppState {
        users: HashMap::new(),
        vehicles: HashMap::new(),
        next_user_id: 0,
        next_vehicle_id: 0,
    };
    let shared_state = Arc::new(Mutex::new(app_state));

    let app = Router::new()
        .route("/users", get(get_users))
        .route("/users/:id", get(get_user))
        .route("/user/add", post(add_user))
        .with_state(shared_state);

    println!("Server starting on http://localhost:3000");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
