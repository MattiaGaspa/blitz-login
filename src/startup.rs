use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{middleware::Logger, web, App, HttpServer};
use redis::Client;

use crate::routes::health_check::health_check;
use crate::routes::add::add;
use crate::routes::edit::edit;
use crate::routes::remove::remove;

pub fn run(listener: TcpListener, redis: Client) -> Result<Server, std::io::Error> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let redis = web::Data::new(redis);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/add", web::post().to(add))
            .route("/remove", web::post().to(edit))
            .route("/edit", web::post().to(remove))
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(redis.clone())
    })
        .listen(listener)?
        .run();
    Ok(server)
}