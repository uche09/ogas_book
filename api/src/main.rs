use ob_api::{config, db, routes::create_router, state::AppState};
use std::env;
use tracing_subscriber::{EnvFilter, fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with(
            tracing_subscriber::fmt::layer()
            .with_writer(std::io::stdout)
            .with_target(true)
            .with_thread_ids(true)
            .with_file(true)
            .with_line_number(true)
            .with_span_events(FmtSpan::CLOSE)
            .pretty()
        )
        .init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pg_pool = db::make_pool(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    let config = config::Config::from_env()?;
    let state = AppState::new(pg_pool);

    // Build router
    let router = create_router(state);
    let addr = format!("0.0.0.0:{}", config.port);

    // Bind to host port and serve
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind listener");

    println!("Server running on http://localhost:{}", config.port);

    axum::serve(listener, router)
        .await
        .expect("Failed to start server");

    Ok(())
}
