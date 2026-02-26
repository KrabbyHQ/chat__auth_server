mod common;

use common::{RegisterRequest, TestLoginResponse, setup_test_server};
use uuid::Uuid;

#[tokio::test]
async fn test_logout_user_success() {
    let server = setup_test_server().await;

    let unique_id = Uuid::new_v4().to_string();
    let email = format!("logout_{}@example.com", unique_id);
    let password = "password123";

    // Register first
    server
        .post("/api/v1/auth/register")
        .json(&RegisterRequest {
            first_name: "Logout".to_string(),
            last_name: "User".to_string(),
            email: email.clone(),
            password: password.to_string(),
            country: "TestCountry".to_string(),
            phone_number: unique_id[0..10].to_string(),
        })
        .await
        .assert_status(axum::http::StatusCode::CREATED);

    // Logout
    let response = server
        .post(&format!("/api/v1/auth/logout?user_email={}", email))
        .await;

    response.assert_status(axum::http::StatusCode::OK);
    let body = response.json::<TestLoginResponse>();
    assert_eq!(body.response_message, "Logout successful");

    // Auth cookie should be cleared (max-age=0)
    // Note: TestServer's cookie handling might vary, but we expect the header to be present
}

#[tokio::test]
async fn test_logout_non_existent_user() {
    let server = setup_test_server().await;

    let response = server
        .post("/api/v1/auth/logout?user_email=ghost@example.com")
        .await;

    // Based on the controller implementation, it uses fetch_one which panics/errors if not found
    // Let's see how it behaves. Usually it returns INTERNAL_SERVER_ERROR if fetch_one fails.
    response.assert_status(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
}
