use actix_web::{web, HttpResponse};
use redis::Client;
use crate::types::Login;

pub async fn add(login: web::Json<Login>, mut redis: web::Data<Client>) -> HttpResponse {
    let mut con = redis.get_ref()
        .get_multiplexed_async_connection()
        .await
        .expect("failed to get redis connection");
    let _: () = redis::cmd("SET")
        .arg("foo")
        .arg("bar")
        .query(&mut con)
        .expect("failed to execute SET for 'foo'");
    HttpResponse::Ok().finish()
}