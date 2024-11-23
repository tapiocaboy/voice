use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use sqlx::PgPool;
use tracing_subscriber::fmt::format::FmtSpan;

mod config;
mod error;
mod handlers;
mod models;
mod services;
mod utils;

use crate::config::Config;
use crate::handlers::{audio, analysis, health};
use crate::models::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv::dotenv().ok();
    
    // Initialize logging
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::CLOSE)
        .with_env_filter("info")
        .init();

    // Load configuration
    let config = Config::from_env()
        .expect("Failed to load configuration");

    // Initialize database connection
    let db_pool = PgPool::connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    // Initialize Redis connection
    let redis_client = redis::Client::open(config.redis_url.clone())
        .expect("Failed to create Redis client");

    // Create shared application state
    let app_state = web::Data::new(AppState {
        db: db_pool,
        redis: redis_client,
        config: config.clone(),
    });

    println!("Starting server at http://localhost:{}", config.port);

    // Start HTTP server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            .service(
                web::scope("/api")
                    .service(health::health_check)
                    .service(
                        web::scope("/audio")
                            .service(audio::upload)
                            .service(audio::process)
                            .service(audio::stream)
                    )
                    .service(
                        web::scope("/analysis")
                            .service(analysis::get_analysis)
                            .service(analysis::get_transcription)
                            .service(analysis::get_emotions)
                    )
            )
    })
    .bind(("0.0.0.0", config.port))?
    .run()
    .await
} 