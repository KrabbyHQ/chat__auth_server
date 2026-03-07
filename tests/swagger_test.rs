mod common;

use common::setup_test_server;
use uuid::Uuid;

#[tokio::test]
async fn test_swagger_endpoint() {
    let server = setup_test_server().await;

    let response = server.get("/swagger-ui/").await;

    response.assert_status(axum::http::StatusCode::OK);
}
