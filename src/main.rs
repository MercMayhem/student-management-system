mod auth;
mod routes;

use std::io::Result;

use actix_web::{web, App, HttpServer};
use routes::{admin::{admin_get_application_details::admin_get_applications, admin_get_details::admin_get_details, admin_post_status::update_application_status, admin_update_details::admin_update_student_details}, user::{apply_application::apply, get_application_details::get_applications, get_details::get_details}, utils::{AdminApplicationsResponse, Application, ApplicationStatusUpdate, ApplicationsResponse, DetailsFields, DetailsQuery, Status, UpdateFields}};
use sqlx::sqlite::SqlitePool;
use utoipa::{openapi::{self, security::{HttpAuthScheme, HttpBuilder, SecurityScheme}}, Modify, OpenApi};
use utoipa_swagger_ui::SwaggerUi;
use crate::routes::user::update_details::update_student_details;


use auth::authentication::{auth_models::{LoginRequest, LoginResponse, RegisterRequest}, login, register};

#[actix_rt::main]
async fn main() -> Result<()>{
    let database_url = "sqlite://college.db";
    let pool = SqlitePool::connect(database_url).await.unwrap();


    #[derive(OpenApi)]
    #[openapi(
            paths(
                auth::authentication::register,
                auth::authentication::login,
                routes::user::update_details::update_student_details,
                routes::admin::admin_update_details::admin_update_student_details,
                routes::user::get_details::get_details,
                routes::admin::admin_get_details::admin_get_details,
                routes::user::apply_application::apply,
                routes::user::get_application_details::get_applications,
                routes::admin::admin_get_application_details::admin_get_applications,
                routes::admin::admin_post_status::update_application_status,
            ),

            components(
                schemas(
                    LoginRequest,
                    LoginResponse,
                    RegisterRequest,
                    DetailsQuery,
                    Application,
                    ApplicationsResponse,
                    Status,
                    ApplicationStatusUpdate,
                    AdminApplicationsResponse,
                    UpdateFields,
                    DetailsFields
                )
            ),

            modifiers(&SecurityAddon)
        )
    ]
    struct ApiDoc;

    struct SecurityAddon;
    impl Modify for SecurityAddon{
        fn modify(&self, openapi: &mut openapi::OpenApi) {
            let components = openapi.components.as_mut().unwrap();
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build()
                )    
            )
        }
    }

    let openapi = ApiDoc::openapi();


    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()))
            .service(register)
            .service(login)
            .service(update_student_details)
            .service(admin_update_student_details)
            .service(get_details)
            .service(admin_get_details)
            .service(apply)
            .service(get_applications)
            .service(admin_get_applications)
            .service(update_application_status) 
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
