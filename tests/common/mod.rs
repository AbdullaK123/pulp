use std::env;

use axum_test::TestServer;
use sqlx::{Connection, PgConnection};
use testcontainers::{runners::AsyncRunner, ContainerAsync, GenericImage};
use testcontainers::core::WaitFor;

use pulp::app::app::create_app;
use pulp::infrastructure::logging::init_logger;

pub struct TestHarness {
    pub server: TestServer,
    _pg: ContainerAsync<GenericImage>,
    pub database_url: String,
}

impl TestHarness {
    pub async fn spawn() -> anyhow::Result<Self> {
        // Init logger once across tests
        init_logger();

        // Start a Postgres 16 container
        let pg_img = GenericImage::new("postgres", "18")
            .with_wait_for(WaitFor::message_on_stderr(
                "database system is ready to accept connections",
            ))
            .with_env_var("POSTGRES_USER", "postgres")
            .with_env_var("POSTGRES_PASSWORD", "password")
            .with_env_var("POSTGRES_DB", "pulp_test");

        let container = pg_img.start().await;

        // Get the mapped port for 5432
        let host_port = container
            .get_host_port_ipv4(5432)
            .await;

        let database_url = format!(
            "postgres://postgres:password@127.0.0.1:{host_port}/pulp_test"
        );

        // Create DB (the db is already created via POSTGRES_DB but ensure connectivity)
        let mut conn = PgConnection::connect(&database_url).await?;
        // Ensure extensions or schema prerequisites if needed
        conn.ping().await?;

        // Run migrations
        sqlx::migrate!("./migrations").run(&mut conn).await?;

        // Set env var so app picks it up (environment changes are unsafe in Rust 2024)
        unsafe { env::set_var("DATABASE_URL", &database_url); }

        // Build the app router
        let app = create_app().await;
        let server = axum_test::TestServer::builder()
            .save_cookies()
            .build(app);

        Ok(Self { server, _pg: container, database_url })
    }
}

// -----------------
// Helper utilities
// -----------------

pub fn unique_email() -> String {
    // Use a random UUID to avoid uniqueness conflicts
    let id = uuid::Uuid::new_v4();
    format!("user+{}@example.com", id)
}

pub async fn register_user(server: &TestServer, email: &str, password: &str) -> axum_test::TestResponse {
    server
        .post("/api/v1/auth/register")
        .json(&serde_json::json!({
            "email": email,
            "password": password
        }))
        .await
}

pub async fn login_user(server: &TestServer, email: &str, password: &str) -> axum_test::TestResponse {
    server
        .post("/api/v1/auth/login")
        .json(&serde_json::json!({
            "email": email,
            "password": password
        }))
        .await
}

pub async fn logout_user(server: &TestServer) -> axum_test::TestResponse {
    server.post("/api/v1/auth/logout").await
}
