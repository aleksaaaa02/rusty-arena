use serde::{Deserialize, Serialize};


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
