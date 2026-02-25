mod common;

use common::{RegisterRequest, TestRegisterResponse, setup_test_server};
use uuid::Uuid;

#[tokio::test]
async fn test_register_user_success() {
    let server = setup_test_server().await;

    let unique_id = Uuid::new_v4().to_string();
    let email = format!("test_{}@example.com", unique_id);
    let phone = unique_id[0..10].to_string();

    let response = server
        .post("/api/v1/auth/register")
        .json(&RegisterRequest {
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            email: email.clone(),
            password: "password123".to_string(),
            country: "TestCountry".to_string(),
            phone_number: phone,
        })
        .await;

    response.assert_status(axum::http::StatusCode::CREATED);

    let body = response.json::<TestRegisterResponse>();
    assert_eq!(
        body.response_message,
        format!("User with email '{}' registered successfully!", email)
    );
    assert!(body.response.unwrap().user_profile.unwrap().id > 0);
}

#[tokio::test]
async fn test_register_user_duplicate_email() {
    let server = setup_test_server().await;

    let unique_id = Uuid::new_v4().to_string();
    let email = format!("dup_{}@example.com", unique_id);
    let phone1 = unique_id[0..10].to_string();
    let phone2 = unique_id[11..21].to_string();

    // First registration
    server
        .post("/api/v1/auth/register")
        .json(&RegisterRequest {
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            email: email.clone(),
            password: "password123".to_string(),
            country: "TestCountry".to_string(),
            phone_number: phone1,
        })
        .await
        .assert_status(axum::http::StatusCode::CREATED);

    // Second registration with same email
    let response = server
        .post("/api/v1/auth/register")
        .json(&RegisterRequest {
            first_name: "Test2".to_string(),
            last_name: "User2".to_string(),
            email,
            password: "password456".to_string(),
            country: "TestCountry".to_string(),
            phone_number: phone2,
        })
        .await;

    response.assert_status(axum::http::StatusCode::FORBIDDEN);
    let body = response.json::<TestRegisterResponse>();
    assert_eq!(body.error.unwrap(), "Email already exists");
}

#[tokio::test]
async fn test_register_user_duplicate_phone_number() {
    let server = setup_test_server().await;

    let unique_id1 = Uuid::new_v4().to_string();
    let email1 = format!("phone_dup1_{}@example.com", unique_id1);
    let phone = unique_id1[0..10].to_string();

    let unique_id2 = Uuid::new_v4().to_string();
    let email2 = format!("phone_dup2_{}@example.com", unique_id2);

    // First registration
    server
        .post("/api/v1/auth/register")
        .json(&RegisterRequest {
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            email: email1,
            password: "password123".to_string(),
            country: "TestCountry".to_string(),
            phone_number: phone.clone(),
        })
        .await
        .assert_status(axum::http::StatusCode::CREATED);

    // Second registration with same phone number but different email
    let response = server
        .post("/api/v1/auth/register")
        .json(&RegisterRequest {
            first_name: "Test2".to_string(),
            last_name: "User2".to_string(),
            email: email2,
            password: "password456".to_string(),
            country: "TestCountry".to_string(),
            phone_number: phone,
        })
        .await;

    response.assert_status(axum::http::StatusCode::FORBIDDEN);
    let body = response.json::<TestRegisterResponse>();
    assert_eq!(body.error.unwrap(), "Phone number already exists");
}
