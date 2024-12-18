use std::sync::Once;

use actix_web::{middleware, web, App, HttpRequest, HttpServer};

mod otel;
use crate::otel::init_otlp;

#[tracing::instrument]
async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {req:?}");
    "Hello world!"
}

static INIT: Once = Once::new();

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    INIT.call_once(|| init_otlp("otel-actix-example"));

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/index.html").to(|| async { "Hello world!" }))
            .service(web::resource("/").to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
