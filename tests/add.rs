use std::net::TcpListener;

use blitz_login::utility::run;
use blitz_login::types::Credentials;

#[tokio::test]
async fn health_check_works() {
    let address = spawn().await;
    let login = Credentials {
        username: "foo".to_string(),
        password: "bar".to_string(),
    };
    let login = serde_json::to_string(&login).expect("Failed to serialize login.");

    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{}/health_check", &address))
        .header("Content-Type", "application/json")
        .body(login)
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
        .expect("Failed to open redis connection.");
    let server = run(listener, redis).expect("Failed to run server.");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}