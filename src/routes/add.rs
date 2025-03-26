use actix_web::{web, HttpResponse};
use redis::Client;

use crate::types::{hash, Credentials};

pub async fn add(login: web::Json<Credentials>, redis: web::Data<Client>) -> HttpResponse {
    let mut con = match redis.get_ref()
        .get_connection() {
        Ok(con) => con,
        Err(e) => {
            log::error!("Failed to get redis connection: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    match redis::cmd("SET")
        .arg(&login.username)
        .arg(hash(&login.password))
        .exec(&mut con) {
        Ok(_) => {
            log::info!("Successfully added user {}.", login.username);
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            log::error!("Failed to add user {}: {}", login.username, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}