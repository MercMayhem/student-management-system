use std::future::ready;
use std::pin::Pin;
use super::common_models::Claims;

use actix_web::error::ErrorUnauthorized;
use actix_web::{web, Error, FromRequest};
use futures::Future;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::sqlite::SqlitePool;

#[derive(Debug, Deserialize, Serialize)]
pub struct AdminUser{
    pub username: String,
    pub password: String,
    pub email: String,
    pub is_admin: bool
}

impl FromRequest for AdminUser {
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    type Error = Error;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        // 1. Check if authorization header is present

        let auth_header_option = req.headers().get("Authorization");
        if auth_header_option.is_none(){
            return Box::pin(ready(Err(ErrorUnauthorized(json!({"error" : "Missing Authorization Headers"})))));
        }

        // 2. Check if authorization is of "Bearer" type

        let auth_header = auth_header_option.unwrap().to_str().unwrap_or("");
        if !auth_header.starts_with("Bearer "){
            return Box::pin(ready(Err(ErrorUnauthorized(json!({"error" : "Invalid Authorization Scheme"})))))
        }

        // 3. Decode token
        
        let token = &auth_header[7..];
        
        let claims_result = decode::<Claims>(token,
                            &DecodingKey::from_secret("test_key".as_ref()), 
                            &Validation::new(Algorithm::HS256));

        if claims_result.is_err() {
            let error = claims_result.unwrap_err().kind().clone();
            match error{
                ErrorKind::ExpiredSignature => return Box::pin(ready(Err(ErrorUnauthorized(json!({"error" : "Expired token. login again."}))))),
                _ => return Box::pin(ready(Err(ErrorUnauthorized(json!({"error" : "Failed to validate JWT token"})))))
            }
        }

        let claims = claims_result.unwrap();

        // 4. Check if user token is not expired and user is an admin

        let pool = req.app_data::<web::Data<SqlitePool>>().unwrap().clone();
        Box::pin(async move {

            let user_record = sqlx::query!(
                    "SELECT * FROM users WHERE email = ?",
                    claims.claims.sub
                )
                .fetch_one(pool.get_ref())
                .await;

            if user_record.is_err(){
                return Err(ErrorUnauthorized(json!({"error" : "Failed to validate JWT token"})))
            }

            let user = user_record.unwrap();

            if user.is_admin.is_none() || user.is_admin == Some(0){
                return Err(ErrorUnauthorized(json!({"error" : "User not authorized to use this resource"})))
            }

            return Ok(AdminUser {
                username: (user.username), 
                password: (user.password.unwrap()), 
                email: (user.email), 
                is_admin: true 
            })
        })
        
    }
}
