mod auth;
mod routes;

use actix_web::{web, App, HttpServer};
use sqlx::sqlite::SqlitePool;

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
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
