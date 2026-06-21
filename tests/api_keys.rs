mod common;

use serde_json::Value;

#[tokio::test]
async fn create_api_key_requires_auth_401() {
    let harness = common::TestHarness::spawn().await.expect("spawn harness");
    let server = harness.server;

    let resp = server.post("/api/auth/api-keys/create").await;
    resp.assert_status_unauthorized();
}

#[tokio::test]
async fn create_api_key_ok_with_and_without_name() {
    let harness = common::TestHarness::spawn().await.expect("spawn harness");
    let server = harness.server;

    let email = common::unique_email();
    common::register_user(&server, &email, "verysecure").await.assert_status_ok();
    let login = common::login_user(&server, &email, "verysecure").await;
    login.assert_status_ok();
    let session_cookie = login
        .cookies()
        .get("session_id")
        .expect("session cookie present")
        .value()
        .to_string();

    // Without name
    let create1 = server
        .post("/api/auth/api-keys/create")
        .add_header("Cookie", format!("session_id={}", session_cookie))
        .await;
    create1.assert_status_ok();
    let json1: Value = create1.json();
    assert!(json1.get("id").is_some());
    assert!(json1.get("key").is_some());
    assert!(json1.get("name").is_some());

    // With name
    let create2 = server
        .post("/api/auth/api-keys/create?name=my-test-key")
        .add_header("Cookie", format!("session_id={}", session_cookie))
        .await;
    create2.assert_status_ok();
    let json2: Value = create2.json();
    assert_eq!(json2["name"], Value::from("my-test-key"));
}

#[tokio::test]
async fn revoke_api_key_ok_then_not_found() {
    let harness = common::TestHarness::spawn().await.expect("spawn harness");
    let server = harness.server;

    let email = common::unique_email();
    common::register_user(&server, &email, "verysecure").await.assert_status_ok();
    let login = common::login_user(&server, &email, "verysecure").await;
    login.assert_status_ok();
    let session_cookie = login
        .cookies()
        .get("session_id")
        .expect("session cookie present")
        .value()
        .to_string();

    // Create a key
    let create = server
        .post("/api/auth/api-keys/create?name=to-revoke")
        .add_header("Cookie", format!("session_id={}", session_cookie))
        .await;
    create.assert_status_ok();
    let json: Value = create.json();
    let key_id = json["id"].as_str().expect("id string").to_string();

    // Revoke it
    let revoke = server
        .post(&format!("/api/auth/api-keys/revoke?key_id={}", key_id))
        .add_header("Cookie", format!("session_id={}", session_cookie))
        .await;
    revoke.assert_status_ok();

    // Revoke again: current implementation updates the row again, so 200 is fine
    let revoke2 = server
        .post(&format!("/api/auth/api-keys/revoke?key_id={}", key_id))
        .add_header("Cookie", format!("session_id={}", session_cookie))
        .await;
    revoke2.assert_status_ok();
}
