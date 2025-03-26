use actix_web::{web, HttpResponse};
use redis::Client;

use crate::types::Credentials;

pub async fn add(login: web::Json<Credentials>, redis: web::Data<Client>) -> HttpResponse {
    let mut con = match redis.get_ref()
        .get_connection() {
        Ok(con) => con,
        Err(e) => {
            log::error!("Failed to get redis connection: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };
    let hashed_credentials = login.hash();

    match redis::cmd("SET")
        .arg(&hashed_credentials.username)
        .arg(&hashed_credentials.password)
        .exec(&mut con) {
        Ok(_) => {
            log::info!("Successfully added user {}.", hashed_credentials.username);
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            log::error!("Failed to add user {}: {}", hashed_credentials.username, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}