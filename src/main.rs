mod auth;
mod routes;

use actix_web::{web, App, HttpServer};
use routes::{admin::{admin_get_application_details::admin_get_applications, admin_get_details::admin_get_details, admin_update_details::admin_update_student_details}, user::{apply_application::apply, get_application_details::get_applications, get_details::get_details}};
use sqlx::sqlite::SqlitePool;
use crate::routes::user::update_details::update_student_details;

use auth::authentication::{login, register};

#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    let database_url = "sqlite://college.db";
    let pool = SqlitePool::connect(database_url).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(register)
            .service(login)
            .service(update_student_details)
            .service(admin_update_student_details)
            .service(get_details)
            .service(admin_get_details)
            .service(apply)
            .service(get_applications)
            .service(admin_get_applications)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
