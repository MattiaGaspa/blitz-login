use actix_web::{web, HttpResponse};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use redis::Client;

use crate::types::{hash, ChangePassword};

pub async fn edit(login: web::Json<ChangePassword>, redis: web::Data<Client>) -> HttpResponse {
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

    let password = login.old_password.clone();
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
            match redis::cmd("SET")
                .arg(&login.username)
                .arg(hash(&login.new_password))
                .exec(&mut con) {
                Ok(_) => {
                    log::info!("Successfully updated user {}'s password.", login.username);
                    HttpResponse::Ok().finish()
                },
                Err(e) => {
                    log::error!("Failed to update user {}'s password: {}", login.username, e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        },
        Ok(Err(_)) => {
            log::warn!("Attempt to change password for {}.", login.username);
            HttpResponse::Unauthorized().finish()
        },
        Err(e) => {
            log::error!("Failed to spawn blocking task: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}