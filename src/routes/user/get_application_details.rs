use actix_web::{get, web, HttpResponse};
use sqlx::{query_as, SqlitePool};

use crate::{auth::user::User, routes::utils::ApplicationsResponse};

#[utoipa::path(
    responses(
        (status=200, description="Got logged in user application details"),
        (status=500, description="Error getting application")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Student"


)]
#[get("/applications")]
pub async fn get_applications(user: User, pool: web::Data<SqlitePool>) -> HttpResponse{
    let id_option: Option<(i64, )> = query_as(
            "SELECT id FROM users WHERE username = ?",
        ).bind(&user.username)
        .fetch_optional(pool.as_ref())
        .await.unwrap_or(None);
    
    match id_option {
        Some(id) => {
            let res_result = sqlx::query!(
                "SELECT Users.username, Applications.content, Applications.status FROM Users RIGHT JOIN Applications ON Users.id = Applications.created_by WHERE Users.id = ?",
                id.0
            ).map(|row|
                ApplicationsResponse{
                    username: row.username,
                    content: row.content,
                    status: {
                        match row.status{
                            0 => "Denied".to_string(),
                            1 => "Approved".to_string(),
                            2 | _ => "Pending".to_string()
                        }
                    }
                }
            ).fetch_all(pool.as_ref())
            .await;
            
            if res_result.is_err(){
                return HttpResponse::InternalServerError().body("Error Getting Applications")
            }

            let res = res_result.unwrap();
            return HttpResponse::Ok().json(&res);
        },
        None => {
            return HttpResponse::InternalServerError().body("Error Getting Applications")
        }
    }
}
