use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub username: String,
    pub nationality: String,
    pub id: u64,
}

#[derive(Deserialize, Debug)]
pub struct CreateUserRequest {
    pub username: String,
    pub nationality: String,
}
impl User {
    pub fn new(username: String, nationality: String, id: u64) -> User {
        User {
            username,
            nationality,
            id,
        }
    }
}
