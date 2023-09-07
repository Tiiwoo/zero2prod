use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let addr = spawn_app();
    // We need to bring in `reqwest`
    // to perforam HTTP requests against our application.
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &addr))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // retrieve the port
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    // return the application address to the caller
    format!("http://127.0.0.1:{}", port)
}
