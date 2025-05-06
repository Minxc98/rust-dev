use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub workspace: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignInUser {
    pub username: String,
    pub email: String,
}