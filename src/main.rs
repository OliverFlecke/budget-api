use std::{error::Error, net::SocketAddr};

use budget_api::run_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let port = std::env::var("PORT")
        .map(|p| p.parse::<usize>().expect("PORT is not a valid integer"))
        .unwrap_or(4000);
    let host: SocketAddr = format!("0.0.0.0:{port}").parse().unwrap();

    run_server(&host).await
}
