use axum::{
    extract::Json as JsonExtractor,
    extract::Path,
    extract::State,
    response::Json,
    routing::{get, post},
};

use crate::models::user::{CreateUserRequest, User};
use crate::models::vehicle::{CreateVehicleRequest, Vehicle};
use crate::state::AppState;
use std::sync::{Arc, Mutex};

pub type SharedState = Arc<Mutex<AppState>>;

pub async fn add_user(
    state: axum::extract::State<SharedState>,
    JsonExtractor(payload): JsonExtractor<CreateUserRequest>,
) -> Json<User> {
    let mut app_state = state.lock().unwrap();

    // Get next ID and increment
    let user_id = app_state.next_user_id;
    app_state.next_user_id += 1;

    // Create and store new user
    let new_user = User::new(payload.username, payload.nationality, user_id);
    app_state.users.insert(user_id, new_user.clone());

    Json(new_user)
}

pub async fn get_users(state: axum::extract::State<SharedState>) -> Json<Vec<User>> {
    let app_state = state.lock().unwrap();
    let users_vec: Vec<User> = app_state.users.values().cloned().collect();
    Json(users_vec)
}

pub async fn get_user(
    state: axum::extract::State<SharedState>,
    axum::extract::Path(user_id): axum::extract::Path<u64>,
) -> Json<Option<User>> {
    let app_state = state.lock().unwrap();
    Json(app_state.users.get(&user_id).cloned())
}

pub async fn add_vehicle(
    state: axum::extract::State<SharedState>,
    JsonExtractor(payload): JsonExtractor<CreateVehicleRequest>,
) -> Json<Vehicle> {
    let mut app_state = state.lock().unwrap();

    if let Some(owner) = app_state.users.get(&payload.owner_id).cloned() {
        let vehicle_id = app_state.next_vehicle_id;
        app_state.next_vehicle_id += 1;

        let new_vehicle = Vehicle::new(payload.maker, owner, vehicle_id);
        app_state.vehicles.insert(vehicle_id, new_vehicle.clone());
        Json(new_vehicle)
    } else {
        panic!("Owner not found") // In a real app, handle this error properly
    }
}

pub async fn get_vehicles(state: axum::extract::State<SharedState>) -> Json<Vec<Vehicle>> {
    let app_state = state.lock().unwrap();
    let vehicles_vec: Vec<Vehicle> = app_state.vehicles.values().cloned().collect();
    Json(vehicles_vec)
}
