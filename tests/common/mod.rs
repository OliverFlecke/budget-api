use budget_api::{auth::config::AuthConfig, App};
use derive_getters::Getters;
use std::net::TcpListener;

#[derive(Debug, Getters)]
pub struct TestApp {
    address: String,
}

pub async fn spawn_app() -> anyhow::Result<TestApp> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");
    let test_app = TestApp {
        address: format!("http://127.0.0.1:{}", listener.local_addr().unwrap().port()),
    };

    let auth_config = AuthConfig::default();
    std::env::set_var("ISSUER", auth_config.issuer());
    std::env::set_var("AUDIENCE", auth_config.audience());

    let server = App::create()
        .await
        .expect("Failed to create app")
        .serve(listener);
    let _ = tokio::spawn(server);

    Ok(test_app)
}
