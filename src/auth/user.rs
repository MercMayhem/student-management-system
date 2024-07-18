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

#[derive(Debug, Serialize, Deserialize)]
pub struct User{
    pub username: String,
    pub password: String,
    pub email: String,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub phone_no: Option<String>,
    pub is_admin: Option<bool>,
    pub roll_no: Option<String>
}

impl FromRequest for User {
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

        // 4. Check if user token is expired or not
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

            let admin: Option<bool>;
            if user.is_admin.is_none(){
                admin = None
            } else if user.is_admin == Some(1){
                admin = Some(true)
            } else {
                admin = Some(false)
            }

            return Ok(User { 
                username: (user.username), 
                password: (user.password), 
                email: (user.email), 
                firstname: (user.firstname), 
                lastname: (user.lastname), 
                phone_no: (user.phone_number), 
                is_admin: (admin), 
                roll_no: (user.roll_no) });
        })
    }
}
