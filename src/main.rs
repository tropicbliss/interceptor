use axum::{
    extract::ConnectInfo,
    handler::Handler,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use clap::Parser;
use std::net::SocketAddr;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "interceptor=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    let app = Router::new().route(
        &format!("/{}", cli.old_path),
        get(|ConnectInfo(addr): ConnectInfo<SocketAddr>| async move {
            tracing::debug!("accepted connection from {:?}", addr);
            Redirect::permanent(&cli.new_path)
        }),
    );
    let app = app.fallback(handler_404.into_service());
    let addr = SocketAddr::from(([127, 0, 0, 1], cli.port));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

#[derive(Parser)]
struct Cli {
    // Path to redirect from
    #[clap(short, long)]
    old_path: String,

    // Website to redirect to
    #[clap(short, long)]
    new_path: String,

    // Port to listen from
    #[clap(short, long, default_value = "3000")]
    port: u16,
}
