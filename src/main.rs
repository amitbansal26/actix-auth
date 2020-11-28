mod config;
mod handlers;

use crate::config::Config;
use crate::handlers::app_config;
use actix_web::{middleware::Logger, App, HttpServer};
use eyre::Result;
use tracing::info;

#[actix_rt::main]
async fn main() -> Result<()> {
    let config = Config::from_env().expect("Server configuration");
    info!("Starting Server at {}:{}", config.host, config.port);
    HttpServer::new(move || App::new().wrap(Logger::default()).configure(app_config))
        .bind(format!("{}:{}", config.host, config.port))
        .expect("Cannot bind to port")
        .run()
        .await?;
    Ok(())
}
