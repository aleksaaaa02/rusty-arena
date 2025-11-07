use serde::{Deserialize, Serialize};

enum AppError {
    BadRequest(String),
    Unauthorized(String),
    NotFound(String),
    Conflict(String),
    Internal(String),
}

#[derive(Deserialize, Serialize, Clone)]
pub struct LoginRequest {
    pub username: String,
    pub password: String
}

#[derive(Deserialize, Serialize, Clone)]
pub struct AuthResponse {
    pub id: i32,    
}

#[derive(Deserialize, Serialize, Clone)]
pub struct RegisterReqeust {
    pub username: String,
    pub password: String
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GetServerListResponse {

}
