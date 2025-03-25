use actix_web::HttpResponse;

pub async fn remove() -> HttpResponse {
    HttpResponse::Ok().finish()
}