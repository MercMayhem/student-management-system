use actix_web::web;
use sqlx::{sqlite::SqliteQueryResult, SqlitePool};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateFields{
    username : String,
    firstname : Option<String>,
    lastname : Option<String>,
    phone_number : Option<String>
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
