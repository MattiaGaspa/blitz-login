use std::net::TcpListener;
use blitz_login::startup::run;
use blitz_login::configuration::get_config;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_config().expect("Failed to read configuration.");
    let listener = TcpListener::bind(
        format!("{}:{}", config.server.host, config.server.port)
    )
        .expect("Failed to bind TCP port.");
    let redis = redis::Client::open(config.redis.connection_string())
        .expect("Failed to open redis connection.");
    run(listener, redis)?.await
}