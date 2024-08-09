use axum::{extract::Query, response::Html};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Params {
    pub a: f64,
    pub b: f64,
}

pub async fn index() -> Html<&'static str> {
    Html("<h1>Simple Calculator</h1><p>Use /add, /subtract, /multiply, or /divide with query parameters `a` and `b`.</p>")
}

pub async fn add(Query(params): Query<Params>) -> Html<String> {
    let result = params.a + params.b;
    Html(format!("<h1>Result: {}</h1>", result))
}

pub async fn subtract(Query(params): Query<Params>) -> Html<String> {
    let result = params.a - params.b;
    Html(format!("<h1>Result: {}</h1>", result))
}

pub async fn multiply(Query(params): Query<Params>) -> Html<String> {
    let result = params.a * params.b;
    Html(format!("<h1>Result: {}</h1>", result))
}

pub async fn divide(Query(params): Query<Params>) -> Html<String> {
    if params.b == 0.0 {
        Html("<h1>Error: Division by zero!</h1>".to_string())
    } else {
        let result = params.a / params.b;
        Html(format!("<h1>Result: {}</h1>", result))
    }
}
