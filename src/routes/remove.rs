use actix_web::{web, HttpResponse};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use redis::Client;

use crate::types::Credentials;

pub async fn remove(login: web::Json<Credentials>, redis: web::Data<Client>) -> HttpResponse {
    let mut con = match redis.get_ref()
        .get_connection() {
        Ok(con) => con,
        Err(e) => {
            log::error!("Failed to get redis connection: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let expected_password_hash: String = match redis::cmd("GET")
        .arg(&login.username)
        .query(&mut con) {
        Ok(password) => password,
        Err(e) => {
            log::error!("Failed to get user {}'s password hash: {}", login.username, e);
            return HttpResponse::InternalServerError().finish();
        }
    };
    let expected_password_hash = PasswordHash::new(&expected_password_hash)
        .expect("Invalid password hash");
    match Argon2::default()
        .verify_password(
            login.password.as_bytes(),
            &expected_password_hash,
        ) {
        Ok(_) => {
            match redis::cmd("DEL")
                .arg(&login.username)
                .exec(&mut con) {
                Ok(_) => {
                    log::info!("Successfully removed user {}.", login.username);
                    HttpResponse::Ok().finish()
                },
                Err(e) => {
                    log::error!("Failed to remove user {}: {}", login.username, e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        },
        Err(_) => {
            log::warn!("Attempt to remove user {}.", login.username);
            HttpResponse::Unauthorized().finish()
        }
    }
}