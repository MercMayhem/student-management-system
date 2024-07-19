use actix_web::{post, web, HttpResponse};
use sqlx::{query, query_as, SqlitePool};

use crate::{auth::user::User, routes::utils::Application};

#[utoipa::path(
    request_body = Application,
    responses(
        (status=200, description="Application uploaded"),
        (status=500, description="Error uploading application")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Student"


)]
#[post("/apply")]
pub async fn apply(user: User, pool: web::Data<SqlitePool>, application: web::Json<Application>) -> HttpResponse {
    let id_option: Option<(i64, )> = query_as(
            "SELECT id FROM users WHERE username = ?",
        ).bind(&user.username)
        .fetch_optional(pool.as_ref())
        .await.unwrap_or(None);


    match id_option {
        Some(id) => {
            let res = query!(
                "INSERT INTO Applications (created_by, content, status) VALUES (?, ?, 2)",
                id.0, application.content
            ).execute(pool.as_ref()).await;

            if res.is_err(){
                return HttpResponse::InternalServerError().body("Error Creating Application")
            }

            return HttpResponse::Ok().body("Successfully created application")
        },
        None => {
            return HttpResponse::InternalServerError().body("Error Creating Application")
        }
    }
}
