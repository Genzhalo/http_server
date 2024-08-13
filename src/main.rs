use axum::Router;
use clap::{command, Parser};
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

#[derive(Parser, Debug)]
#[command(name = "Static File Server")]
#[command(version = "1.0")]
#[command(about = "A simple static file server")]
struct Args {
    #[arg(short, long, default_value_t = 8000)]
    port: u16,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();

    let app = Router::new()
        .nest_service("/", ServeDir::new("."))
        .fallback_service(ServeFile::new("index.html"))
        .layer(
            TraceLayer::new_for_http()
                .on_response(DefaultOnResponse::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO)),
        );

    println!("{:?}", args);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", args.port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
