use actix_web::web;
use sqlx::{sqlite::SqliteQueryResult, SqlitePool};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateFields{
    #[schema(example = "John Doe", required = true)]
    username : String,
    #[schema(example = "John", required = false)]
    firstname : Option<String>,
    #[schema(example = "Doe", required = false)]
    lastname : Option<String>,
    #[schema(example = "9532992942", required = false)]
    phone_number : Option<String>
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DetailsFields{
    #[schema(example = "example@gmail.com")]
    pub email: String,
    #[schema(example = "John Doe")]
    pub username: String,
    #[schema(example = "John")]
    pub firstname : Option<String>,
    #[schema(example = "Doe")]
    pub lastname : Option<String>,
    #[schema(example = "9532992942")]
    pub phone_number : Option<String>,
    #[schema(example = "21CTY3334")]
    pub roll_no: Option<String>
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct DetailsQuery{
    #[schema(example = "4", required = false)]
    pub page: Option<i64>,
    #[schema(example = "30", required = false)]
    pub num: Option<i64>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct Application{
    #[schema(example = "Application for a medical leave", required = true)]
    pub content: String
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ApplicationsResponse{
    #[schema(example = "John Doe")]
    pub username: String,
    #[schema(example = "Application for a medical leave...")]
    pub content: String,
    #[schema(example = "Approved")]
    pub status: String
}

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example="Approved")]
pub enum Status{
    Denied,
    Approved,
    Pending
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ApplicationStatusUpdate{
    #[schema(example = "7", required = true)]
    pub application_id: i64,
    #[schema(example = "Denied", required = true)]
    pub status: Status
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AdminApplicationsResponse{
    #[schema(example = "7")]
    pub id: i64,
    #[schema(example = "John Doe")]
    pub username: String,
    #[schema(example = "example@gmail.com")]
    pub email: String,
    #[schema(example = "9532992942")]
    pub phone_number: Option<String>,
    #[schema(example = "Approved")]
    pub roll_no: Option<String>,
    #[schema(example = "Application for a medical leave...")]
    pub content: String,
    #[schema(example = "21CTY3334")]
    pub status: String
}

pub async fn update_from_email(pool: web::Data<SqlitePool>, fields: web::Json<UpdateFields>, user_email: &String) -> sqlx::Result<SqliteQueryResult>{
    sqlx::query!(
        "update users set username = ?, firstname = ?, lastname = ?, phone_number = ? where email = ?;",
        fields.username,
        fields.firstname,
        fields.lastname,
        fields.phone_number,
        user_email
    ).execute(pool.as_ref()).await
}

pub async fn update_from_id(pool: web::Data<SqlitePool>, fields: web::Json<UpdateFields>, user_id: i64) -> sqlx::Result<SqliteQueryResult>{
    sqlx::query!(
        "update users set username = ?, firstname = ?, lastname = ?, phone_number = ? where id = ?;",
        fields.username,
        fields.firstname,
        fields.lastname,
        fields.phone_number,
        user_id
    ).execute(pool.as_ref()).await
}
