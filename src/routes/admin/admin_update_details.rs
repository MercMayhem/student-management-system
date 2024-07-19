
use actix_web::{patch, web, HttpResponse, Responder};
use sqlx::SqlitePool;

use crate::auth::admin::AdminUser;
use super::super::utils::{update_from_id, UpdateFields};


#[patch("/admin/update/{user_id}")]
async fn admin_update_student_details(pool: web::Data<SqlitePool>, fields: web::Json<UpdateFields>, _user: AdminUser, user_id: web::Path<i64>) -> impl Responder {
    let query_result = update_from_id(pool, fields, *user_id.as_ref()).await;
    match query_result {
        Ok(_) => {
            HttpResponse::Ok().body(format!("Updated Student {user_id} Details"))
        },

        Err(_) => {
            HttpResponse::InternalServerError().body(format!("Could Not Update Student {user_id} Details"))
        }
    }
}
