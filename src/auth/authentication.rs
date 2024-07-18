mod auth_models;
use auth_models::{LoginRequest, LoginResponse, RegisterRequest};
use super::common_models::Claims;

use crate::auth::admin::AdminUser;

use actix_web::error::ErrorUnauthorized;
use actix_web::{web, Error, HttpResponse};
use actix_web::post;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey};
use serde_json::json;
use sqlx::sqlite::SqlitePool;

#[post("/register")]
pub async fn register(
    pool: web::Data<SqlitePool>,
    register_data: web::Json<RegisterRequest>,
    _admin_user: AdminUser
) -> Result<HttpResponse, Error> {
    let hashed_password = hash(&register_data.password, DEFAULT_COST).unwrap();
    
    let result = sqlx::query!(
        "INSERT INTO users (username, email, password, is_admin) VALUES (?, ?, ?, ?)",
        register_data.username,
        register_data.email,
        hashed_password,
        0
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => Ok(HttpResponse::Ok().json("User registered successfully")),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({"error" : format!("Failed to register user: {}", e )}))),
    }
}

#[post("/login")]
pub async fn login(
    pool: web::Data<SqlitePool>,
    login_data: web::Json<LoginRequest>
) -> Result<HttpResponse, Error> {

    let user_result = sqlx::query!(
        "SELECT username, email, password, is_admin from users where email = ?",
        login_data.email
    )
    .fetch_one(pool.as_ref())
    .await;

    match user_result{
            Ok(user_record) => {

                if verify(&login_data.password, &user_record.password).unwrap(){
                    let expiration = chrono::Utc::now()
                                    .checked_add_days(chrono::Days::new(1))
                                    .expect("valid timestamp")
                                    .timestamp() as usize;
                    
                    let claims = Claims{
                        sub: user_record.email.clone(),
                        exp: expiration
                    };
                    let token = encode(
                        &jsonwebtoken::Header::default(),
                        &claims,
                        &EncodingKey::from_secret("test_key".as_ref()), // Use a proper secret key
                    ).unwrap();
                    return Ok(HttpResponse::Ok().json(LoginResponse { token }))
                }

                return Err(ErrorUnauthorized(json!({"error" : "Invalid credentials"})));
            },

            Err(_) => {
                return Err(ErrorUnauthorized(json!({"error" : "Invalid credentials"})));
            }
    }
}
