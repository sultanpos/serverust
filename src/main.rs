use dotenvy::dotenv;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let app = sultan::create_app().await?;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001").await?;

    info!("Server listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}
