use axum::{extract::Query, response::Html, routing::get, Router};
use serde::Deserialize;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // route time!
    let app = Router::new()
        .route("/", get(index))
        .route("/add", get(add))
        .route("/subtract", get(subtract))
        .route("/multiply", get(multiply))
        .route("/divide", get(divide));

    // Run it
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// Index page
async fn index() -> Html<&'static str> {
    Html("<h1>Simple Calculator</h1><p>Use /add, /subtract, /multiply, or /divide with query parameters `a` and `b`.</p>")
}

// gonna make a struct for each one
#[derive(Deserialize)]
struct Params {
    a: f64,
    b: f64,
}

// Lets define the logic here!
async fn add(Query(params): Query<Params>) -> Html<String> {
    let result = params.a + params.b;
    Html(format!("<h1>Result: {}</h1>", result))
}

async fn subtract(Query(params): Query<Params>) -> Html<String> {
    let result = params.a - params.b;
    Html(format!("<h1>Result: {}</h1>", result))
}

async fn multiply(Query(params): Query<Params>) -> Html<String> {
    let result = params.a * params.b;
    Html(format!("<h1>Result: {}</h1>", result))
}

async fn divide(Query(params): Query<Params>) -> Html<String> {
    if params.b == 0.0 {
        Html("<h1>Error: Division by zero!</h1>".to_string())
    } else {
        let result = params.a / params.b;
        Html(format!("<h1>Result: {}</h1>", result))
    }
}
