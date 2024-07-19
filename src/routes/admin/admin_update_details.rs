
use actix_web::{patch, web, HttpResponse, Responder};
use sqlx::SqlitePool;

use crate::auth::admin::AdminUser;
use super::super::utils::{update_from_id, UpdateFields};

#[utoipa::path(
    params(
        ("user_id", description="id of student whose details are being updated")
    ),
    responses(
        (status=200, description="Updated application status"),
        (status=500, description="Error updating application status")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Admin"


)]
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
