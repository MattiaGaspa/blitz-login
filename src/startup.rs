use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use crate::routes::health_check::health_check;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            // .route("/add", web::post().to())
            // .route("/remove", web::post().to())
            // .route("/edit", web::post().to())
    })
        .listen(listener)?
        .run();
    Ok(server)
}