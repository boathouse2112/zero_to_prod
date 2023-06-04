use std::net::TcpListener;

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

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app().expect("Failed to spawn app");
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_response_for_valid_form_data() {
    // Arrange
    let address = spawn_app().expect("Failed to spawn app");
    let client = reqwest::Client::new();

    // Act
    let body = "name=Le%20Guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(format!("{}/subscriptions", address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_response_when_data_is_missing() {
    // Arrange
    let address = spawn_app().expect("Failed to spawn app");
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_msg) in test_cases {
        // Act
        let response = client
            .post(format!("{}/subscriptions", address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The api did not fail with 400 Bad Request when the payload was {}.",
            error_msg
        );
    }
}
