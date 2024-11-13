use reqwest::Client;
use std::process::{Command, Stdio};
use tokio::time::Duration;
use tokio::time::sleep;
use std::io::{BufRead, BufReader};

#[tokio::test]
async fn user_no_auth_forbidden() {
    // Start the server in the background by running the `cargo run` command
    let mut server = Command::new("cargo")
        .arg("run")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start the server");

    // Get the stdout of the cargo process to monitor server startup
    let stdout = BufReader::new(server.stdout.take().expect("Failed to capture stdout"));

    // Wait for the "Listening on ..." message to appear in the output
    let server_startup_message = "Starting server at http://127.0.0.1";
    let mut server_ready = false;

    for line in stdout.lines() {
        if let Ok(line) = line {
            if line.contains(server_startup_message) {
                server_ready = true;
                break;
            }
        }
    }

    // Ensure the server is ready before sending requests
    assert!(server_ready, "Server did not start up properly");

    // Wait a moment for the server to start
    sleep(Duration::from_millis(100)).await;

    // Create a reqwest client to send requests
    let client = Client::new();

    // Send a GET request to the root endpoint
    let response = client.get("http://127.0.0.1:8080/user")
        .send()
        .await
        .expect("Failed to send GET request");

    // Check the response body
    let body = response.text().await.expect("Failed to read response body");
    assert_eq!(body, "Forbidden");

    // Stop the server after the test
    let _ = server.kill();

    // Wait for the server to completely exit (important for ensuring the port is released)
    let _ = server.wait().expect("Failed to wait for the server process to terminate");
}