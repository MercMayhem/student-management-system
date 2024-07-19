use actix_web::{patch, web, HttpResponse, Responder};
use sqlx::SqlitePool;

use crate::auth::user::User;
use super::super::utils::{update_from_email, UpdateFields};


#[patch("/update")]
async fn update_student_details(pool: web::Data<SqlitePool>, fields: web::Json<UpdateFields>, user: User) -> impl Responder {
    let query_result = update_from_email(pool, fields, &user.email).await;
    match query_result {
        Ok(_) => {
            HttpResponse::Ok().body("Updated Student Details")
        },

        Err(_) => {
            HttpResponse::InternalServerError().body("Could Not Update Student Details")
        }
    }
}
