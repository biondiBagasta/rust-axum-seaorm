use sea_orm::{Database, DatabaseConnection};
use tokio::net::TcpListener;
use axum::{routing::{delete, get, post, put}, Router};

mod model;
mod controller;

use controller::{category_controller, product_controller, user_controller};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to load .env file.");

    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("localhost:3000".to_owned());
    let database_url = std::env::var("DATABASE_URL").expect("Database Url from .env file not found.");

    let db: DatabaseConnection = Database::connect(database_url).await.expect("Failed to Connect to the Database");
    // let db = Arc::new(db); // Use Arc to allow cloning in handlers

    let listener = TcpListener::bind(server_address)
    .await.expect("Couldn't create TCP Listener.");

    print!("Listening on {} ", listener.local_addr().unwrap());

    let app_router = Router::new()
    .route("/api", get(|| async { "Hello World" }))
    // Category Route
    .route("/api/category", post(category_controller::create))
    .route("/api/category/many", get(category_controller::find_many))
    .route("/api/category/search", post(category_controller::search_paginate))
    .route("/api/category/first/{id}", get(category_controller::find_first))
    .route("/api/category/{id}", put(category_controller::update))
    .route("/api/category/{id}", delete(category_controller::delete))

    // Product Route
    .route("/api/product/search", post(product_controller::search_paginate))
    .route("/api/product", post(product_controller::create))
    .route("/api/product/{id}", put(product_controller::update))
    .route("/api/product/{id}", delete(product_controller::delete))

    // User Route
    .route("/api/user/many", get(user_controller::find_many))
    .route("/api/user", post(user_controller::create))
    .route("/api/user/{id}", put(user_controller::update))
    .route("/api/user/{id}", delete(user_controller::delete))
    .with_state(db);


    axum::serve(listener, app_router).await.expect("Error while serving the server.");
}
