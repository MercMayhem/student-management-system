use actix_web::{get, web, HttpResponse, Responder};
use sqlx::SqlitePool;

use crate::{auth::admin::AdminUser, routes::utils::{DetailsFields, DetailsQuery}};

#[utoipa::path(
    request_body = DetailsQuery,
    responses(
        (status=200, description="Get student details", body=DetailsFields),
        (status=500, description="Error getting details")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Admin"


)]
#[get("/admin/details")]
async fn admin_get_details(path: web::Path<DetailsQuery>, pool: web::Data<SqlitePool>, _admin: AdminUser) -> impl Responder{
    let page: i64;
    match path.page{
        Some(n) => {page = n - 1},
        None => {page = 0}
    }

    let num: i64;
    match path.num{
        Some(n) => {num = n},
        None => {num = 10}
    }

    let mut ret: Vec<DetailsFields> = Vec::new();
    let start = num * page;
    let query_result = sqlx::query!(
            "SELECT * FROM users WHERE is_admin = 0 LIMIT ?, ?",
            start,
            num
        ).fetch_all(pool.as_ref())
        .await;

    if query_result.is_err(){
        return HttpResponse::InternalServerError().body("Unable to retrieve student details")
    }

    let records = query_result.unwrap();
    for record in records{
        let temp = DetailsFields{ email: record.email, username: record.username,  firstname: record.firstname, lastname: record.lastname, phone_number: record.phone_number, roll_no: record.roll_no };
        ret.push(temp);
    }

    return HttpResponse::Ok().json(ret);
}
