use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app().expect("Failed to spawn app.");
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> std::io::Result<String> {
    // Binding to port 0 makes the OS scan for an available port
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr().unwrap().port();
    let server = zero_to_prod::run(listener)?;
    // Launch the server as a background task
    tokio::spawn(server);

    let address = format!("http://127.0.0.1:{}", port);
    Ok(address)
}
