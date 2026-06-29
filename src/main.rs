use actix_web::{web, App, HttpServer, HttpResponse, middleware::Logger};
use serde::Serialize;
use std::env;

#[derive(Serialize)]
struct InfoResponse {
    service: &'static str,
    version: &'static str,
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

async fn info() -> HttpResponse {
    let response = InfoResponse {
        service: "appengine-rust",
        version: env!("CARGO_PKG_VERSION"),
    };
    HttpResponse::Ok().json(response)
}

async fn health() -> HttpResponse {
    let response = HealthResponse { status: "healthy" };
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("{}:{}", host, port);

    let server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(info))
            .route("/health", web::get().to(health))
    })
    .bind(&addr)?;

    let server_future = server.run();
    let server_handle = server_future.handle();

    actix_web::rt::spawn(async move {
        actix_web::rt::signal::ctrl_c().await.unwrap();
        tracing::info!("Received SIGINT, shutting down gracefully...");
        server_handle.stop(true).await;
    });

    server_future.await
}
