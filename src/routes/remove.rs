use actix_web::{web, HttpResponse};

use crate::types::Login;

pub async fn remove(login: web::Json<Login>) -> HttpResponse {
    HttpResponse::Ok().finish()
}