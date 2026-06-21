mod common;
use serde_json::Value;

#[tokio::test]
async fn health_ok() {
    let harness = common::TestHarness::spawn().await.expect("spawn harness");
    let server = harness.server;

    let resp = server.get("/health").await;
    resp.assert_status_ok();

    let json: Value = resp.json();
    assert_eq!(json["status"], Value::from("healthy"));
    assert_eq!(json["service"], Value::from("pulp-api"));
}
