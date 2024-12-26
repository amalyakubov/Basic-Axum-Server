use super::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vehicle {
    pub maker: String,
    pub owner: User,
}

#[derive(Deserialize, Debug)]
pub struct CreateVehicleRequest {
    pub maker: String,
    pub owner_id: u64,
}

impl Vehicle {
    pub fn new(maker: String, owner: User, _id: u64) -> Vehicle {
        Vehicle { maker, owner }
    }
    pub fn get_owner(&self) -> User {
        self.owner.clone()
    }
}
