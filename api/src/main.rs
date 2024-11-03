#[allow(dead_code)]
#[allow(unused_imports)]
mod api;

use api::db::connect_db;
use api::upload::init_routes;

use actix_web::{middleware::Logger, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set environment variables for logging
    std::env::set_var("RUST_LOG", "debug,actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // Connect to the database
    match connect_db().await {
        Ok(_) => {
            println!("Successfully connected to the database");
            // You can pass the database connection to your routes if needed
        }
        Err(e) => {
            eprintln!("Failed to connect to database: {}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to connect to database",
            ));
        }
    }

    // Start the HTTP server
    HttpServer::new(|| {
        let logger = Logger::default();
        App::new().wrap(logger).configure(init_routes) // Initialize routes
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
