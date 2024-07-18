use actix_web::{get, HttpResponse, Responder};

use crate::{auth::user::User, routes::utils::DetailsFields};

#[get("/details")]
async fn get_details(user: User) -> impl Responder{
    let user_details = DetailsFields{
        email: user.email,
        username: user.username,
        firstname: user.firstname,
        lastname: user.lastname,
        phone_number: user.phone_no,
        roll_no: user.roll_no
    };

    HttpResponse::Ok().json(user_details)
}
