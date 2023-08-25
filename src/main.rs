use std::net::TcpListener;

use budget_api::App;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let port = std::env::var("PORT")
        .map(|p| p.parse::<usize>().expect("PORT is not a valid integer"))
        .unwrap_or(4000);
    let listener = TcpListener::bind(format!("0.0.0.0:{port}"))?;

    App::create().await?.serve(listener).await?;

    Ok(())
}
