use actix_web::{get, web, HttpResponse};
use sqlx::{query, SqlitePool};

use crate::{auth::admin::AdminUser, routes::utils::{AdminApplicationsResponse, DetailsQuery}};

#[get("/admin/studentapplications")]
pub async fn admin_get_applications(_admin: AdminUser, pool: web::Data<SqlitePool>, details: web::Query<DetailsQuery>) -> HttpResponse{
    let page = details.page.unwrap_or(1) - 1;
    let num = details.num.unwrap_or(10);
    let offset = num * page;

    let result_result = query!(
            "SELECT Applications.id, Users.username, Users.email, Users.phone_number, Users.roll_no, Applications.content, Applications.status
                FROM Applications LEFT JOIN Users ON Applications.created_by = Users.id
                LIMIT ?, ?",
                offset,
                num
        ).map(|row|
            AdminApplicationsResponse{
                id: row.id,
                username: row.username.unwrap(),
                email: row.email.unwrap(),
                phone_number: row.phone_number,
                roll_no: row.roll_no,
                content: row.content,
                status: {
                    match row.status{
                        0 => "Denied".to_string(),
                        1 => "Approved".to_string(),
                        2 | _ => "Pending".to_string()
                    }
                }
            }
        ).fetch_all(pool.as_ref()).await;

    if result_result.is_err(){
        return HttpResponse::InternalServerError().body("Error Getting Applications")
    }

    let result = result_result.unwrap();
    return HttpResponse::Ok().json(&result);
}
