use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct User {
    pub id: i32,
    pub github_id: String,
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub github_id: String,
    pub username: String,
}
