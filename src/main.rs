mod quick_dev;
mod routes;

use axum::{routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Initialize quick development utilities
    quick_dev::init_logger();

    // Build our application with routes
    let app = Router::new()
        .route("/", get(routes::calculator::index))
        .route("/add", get(routes::calculator::add))
        .route("/subtract", get(routes::calculator::subtract))
        .route("/multiply", get(routes::calculator::multiply))
        .route("/divide", get(routes::calculator::divide));

    // Run the server
    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    tokio::spawn(axum::serve(listener, app));

    // Test endpoints after server starts
    quick_dev::test_endpoints().await;
}
