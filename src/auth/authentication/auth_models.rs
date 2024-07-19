use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct LoginRequest{
    #[schema(example = "example@gmail.com", required = true)]
    pub email: String,
    #[schema(example = "testpassword@443**2", required = true)]
    pub password: String
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct LoginResponse{
    #[schema(example = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJhbWFucmFvMDMyQGdtYWlsLmNvbSIsImV4cCI6MTcyMTQ5MjM3NH0.MC4LSIyk8xPRPm6YVexZjU5S7VUD0ZLiObEGxvgCqnE")]
    pub token: String
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct RegisterRequest{
    #[schema(example = "John Doe", required = true)]
    pub username: String,
    #[schema(example = "testpassword@443**2", required = true)]
    pub password: String,
    #[schema(example = "example@gmail.com", required = true)]
    pub email: String,
    #[schema(example = "21CTY3334", required = true)]
    pub roll_no: String
}
