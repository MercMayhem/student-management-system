use actix_web::web;
use sqlx::{sqlite::SqliteQueryResult, SqlitePool};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct UpdateFields{
    username : String,
    firstname : Option<String>,
    lastname : Option<String>,
    phone_number : Option<String>
}

#[derive(Debug, Serialize)]
pub struct DetailsFields{
    pub email: String,
    pub username: String,
    pub firstname : Option<String>,
    pub lastname : Option<String>,
    pub phone_number : Option<String>,
    pub roll_no: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct DetailsQuery{
    pub page: Option<i64>,
    pub num: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct Application{
    pub content: String
}

#[derive(Debug, Serialize)]
pub struct ApplicationsResponse{
    pub username: String,
    pub content: String,
    pub status: String
}

#[derive(Debug, Serialize)]
pub struct AdminApplicationsResponse{
    pub id: i64,
    pub username: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub roll_no: Option<String>,
    pub content: String,
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
