use actix_web::{web, HttpResponse};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use redis::Client;

use crate::types::Credentials;

pub async fn login(login: web::Json<Credentials>, redis: web::Data<Client>) -> HttpResponse {
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

    let password = login.password.clone();
    match tokio::task::spawn_blocking(move || {
        let expected_password_hash = PasswordHash::new(&expected_password_hash)
            .expect("Invalid password hash");

        Argon2::default()
            .verify_password(
                password.as_bytes(),
                &expected_password_hash,
            )
    }).await {
        Ok(Ok(_)) => {
            log::info!("User {} has logged in.", login.username);
            HttpResponse::Ok().finish()
        },
        Ok(Err(_)) => {
            log::warn!("Failed login attempt for {}.", login.username);
            HttpResponse::Unauthorized().finish()
        },
        Err(e) => {
            log::error!("Failed to spawn blocking task: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}