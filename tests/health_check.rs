use std::net::TcpListener;

use blitz_login::utility::run;

#[tokio::test]
async fn health_check_works() {
    let address = spawn().await;

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port.");
    let port = listener.local_addr().unwrap().port();

    let redis = redis::Client::open("redis://redis/")
        .unwrap()
        .get_multiplexed_async_connection()
        .await
        .expect("Failed to open redis connection.");
    let server = run(listener, redis).expect("Failed to run server.");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}