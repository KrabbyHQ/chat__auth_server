mod common;

use common::setup_test_server;
use uuid::Uuid;

#[tokio::test]
async fn test_health_check_endpoint() {
    let server = setup_test_server().await;

    let response = server.get("/api/v1/auth/health").await;

    response.assert_status(axum::http::StatusCode::OK);
}
