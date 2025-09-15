use axum::{Router};
use dotenvy::dotenv;
use tokio::net::TcpListener;

pub mod routes;
pub mod models;
pub mod error;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app=Router::new()
        .nest("/api", routes::app_routes());

    let route = "0.0.0.0:8000";

    println!("Listening at http://{}", route);

    let listener = TcpListener::bind(route)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Server failed");

}
