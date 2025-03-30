use std::sync::Mutex;
use actix_web::{web, HttpResponse};
use redis::aio::MultiplexedConnection;

use crate::types::{hash, Credentials};

pub async fn add(login: web::Json<Credentials>, redis: web::Data<Mutex<MultiplexedConnection>>) -> HttpResponse {
    let mut redis = redis.lock().unwrap();
    match redis.send_packed_command(
        redis::cmd("SET")
            .arg(&login.username)
            .arg(hash(&login.password))
    ).await {
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