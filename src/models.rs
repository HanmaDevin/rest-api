use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize)]
pub struct UserPayload {
    name: String,
    email: String,
}

impl UserPayload {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }
}

#[derive(Serialize, FromRow)]
pub struct User {
    id: i32,
    name: String,
    email: String,
}
