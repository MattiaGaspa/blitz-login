use actix_web::HttpResponse;

pub async fn add() -> HttpResponse {
    HttpResponse::Ok().finish()
}