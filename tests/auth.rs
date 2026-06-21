mod common;

use serde_json::Value;

#[tokio::test]
async fn register_valid_creates_user() {
    let harness = common::TestHarness::spawn().await.expect("spawn harness");
    let server = harness.server;

    let email = common::unique_email();
    let resp = common::register_user(&server, &email, "verysecure").await;
    resp.assert_status_ok();

    let json: Value = resp.json();
    assert!(json.get("id").is_some());
    assert_eq!(json["email"], Value::from(email));
}

#[tokio::test]
async fn login_sets_session_and_allows_authenticated_action() {
    let harness = common::TestHarness::spawn().await.expect("spawn harness");
    let server = harness.server;

    let email = common::unique_email();
    // Register first
    let reg = common::register_user(&server, &email, "verysecure").await;
    reg.assert_status_ok();

    // Login
    let login = common::login_user(&server, &email, "verysecure").await;
    login.assert_status_ok();
    // Extract session cookie explicitly
    let session_cookie = login
        .cookies()
        .get("session_id")
        .expect("session cookie present")
        .value()
        .to_string();

    // Now an authenticated endpoint should work: create API key
    let create = server
        .post("/api/auth/api-keys/create")
        .add_header("Cookie", format!("session_id={}", session_cookie))
        .await;
    create.assert_status_ok();

    // Response contains id and key
    let json: Value = create.json();
    assert!(json.get("id").is_some());
    assert!(json.get("key").is_some());
}

#[tokio::test]
async fn logout_clears_session() {
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

    // Logout
    let logout = server
        .post("/api/auth/logout")
        .add_header("Cookie", format!("session_id={}", session_cookie))
        .await;
    logout.assert_status_ok();

    // Authenticated call should now fail with 401
    let create = server
        .post("/api/auth/api-keys/create")
        .add_header("Cookie", format!("session_id={}", session_cookie))
        .await;
    create.assert_status_unauthorized();
}
