use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::FmtSubscriber;


mod routes;
mod engine;
// mod error;
// mod actions;
use routes::create_routes;

pub async fn run()  -> Result<(), Box<dyn std::error::Error>>  {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let app = create_routes();

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    info!("->> server is running on 3000 port");

    axum::serve(listener, app).await?;

    Ok(())
}