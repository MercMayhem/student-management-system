use actix_web::{post, web, HttpResponse};
use sqlx::{query, SqlitePool};

use crate::{auth::admin::AdminUser, routes::utils::ApplicationStatusUpdate};

#[utoipa::path(
    request_body = ApplicationStatusUpdate,
    responses(
        (status=200, description="Updated application status"),
        (status=500, description="Error updating application status")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Admin"


)]
#[post("/admin/update/applicationstatus")]
pub async fn update_application_status(_admin: AdminUser, pool: web::Data<SqlitePool>, update: web::Json<ApplicationStatusUpdate>) -> HttpResponse {
    let status = {
        match update.status{
           crate::routes::utils::Status::Denied => 0,
           crate::routes::utils::Status::Approved => 1,
           crate::routes::utils::Status::Pending => 2
        }
    };

    let id = update.application_id;

    let result = query!(
            "UPDATE Applications SET status = ? WHERE id = ?",
            status,
            id
        ).execute(pool.as_ref()).await;
    
    if result.is_err(){
        return HttpResponse::InternalServerError().body(format!("Unable to Update Status of application: {}", update.application_id))
    }

    
    return HttpResponse::Ok().body(format!("Updated Status of application: {}", update.application_id))
}
