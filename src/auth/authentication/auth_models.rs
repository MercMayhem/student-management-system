use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest{
    pub email: String,
    pub password: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse{
    pub token: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterRequest{
    pub username: String,
    pub password: String,
    pub email: String,
    pub roll_no: String
}
