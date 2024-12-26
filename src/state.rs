use crate::models::user::User;
use crate::models::vehicle::Vehicle;
use std::collections::HashMap;

pub struct AppState {
    pub users: HashMap<u64, User>,
    pub vehicles: HashMap<u64, Vehicle>,
    pub next_user_id: u64,
    pub next_vehicle_id: u64,
}
