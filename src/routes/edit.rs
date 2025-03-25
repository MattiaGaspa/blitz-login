use actix_web::{web, HttpResponse};

use crate::types::Login;

pub async fn edit(login: web::Json<Login>) -> HttpResponse {
    HttpResponse::Ok().finish()
}