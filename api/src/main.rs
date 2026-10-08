use ob_api::{config, routes::create_router};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or("my_api=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = config::Config::from_env()?;

    // Build router
    let router = create_router();
    let addr = format!("0.0.0.0:{}", config.port);

    // Bind to host port and serve
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Faild to bind listener");

    println!("Server running on http://localhost:{}", config.port);

    axum::serve(listener, router)
        .await
        .expect("Failed to start server");
    
    Ok(())
}
