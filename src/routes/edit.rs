use actix_web::{web, HttpResponse};
use redis::Client;

use crate::types::ChangePassword;

pub async fn edit(login: web::Json<ChangePassword>, redis: web::Data<Client>) -> HttpResponse {
    let mut con = match redis.get_ref()
        .get_connection() {
        Ok(con) => con,
        Err(e) => {
            log::error!("Failed to get redis connection: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };
    let hashed_credentials = login.hash();

    let expected_password_hash: String = match redis::cmd("GET")
        .arg(&login.username)
        .query(&mut con) {
        Ok(password) => password,
        Err(e) => {
            log::error!("Failed to get user {}'s password hash: {}", login.username, e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    if expected_password_hash == hashed_credentials.old_password {
        match redis::cmd("SET")
            .arg(&hashed_credentials.username)
            .arg(&hashed_credentials.new_password)
            .exec(&mut con) {
            Ok(_) => {
                log::info!("Successfully updated user {}'s password.", hashed_credentials.username);
                HttpResponse::Ok().finish()
            },
            Err(e) => {
                log::error!("Failed to update user {}'s password: {}", hashed_credentials.username, e);
                HttpResponse::InternalServerError().finish()
            }
        }
    }
    else {
        HttpResponse::Unauthorized().finish()
    }
}