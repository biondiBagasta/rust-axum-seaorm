use sea_orm::{Database, DatabaseConnection};
use tokio::net::TcpListener;
use axum::{middleware, routing::{delete, get, post, put}, Router};
use tower_http::cors::{ Any, CorsLayer };

mod model;
mod controller;
mod utils;

use controller::{
    category_controller, 
    product_controller, 
    user_controller, 
    auth_controller, 
    files_controller 
};

use utils::router_gurard::auth_guard;

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

    let cors = CorsLayer::new().allow_origin(Any);

    let category_router = Router::new()
    .route("/api/category/search-paginate", post(category_controller::search_paginate))
    .route("/api/category", get(category_controller::find_many))
    .route("/api/category/{id}", get(category_controller::find_first))
    .route("/api/category", post(category_controller::create))
    .route("/api/category/{id}", put(category_controller::update))
    .route("/api/category/{id}", delete(category_controller::delete))
    .route_layer(middleware::from_fn(auth_guard));

    let login_router = Router::new()
    .route("/api/auth/login", post(auth_controller::login));

    let auth_router = Router::new()
    .route("/api/auth/authenticated", post(auth_controller::authenticated))
    .route("/api/auth/change-password", post(auth_controller::change_password));

    let product_router = Router::new()
    .route("/api/product/search", post(product_controller::search_paginate))
    .route("/api/product", post(product_controller::create))
    .route("/api/product/{id}", put(product_controller::update))
    .route("/api/product/{id}", delete(product_controller::delete))
    .route_layer(middleware::from_fn(auth_guard));

    let user_router = Router::new()
    .route("/api/user/many", get(user_controller::find_many))
    .route("/api/user", post(user_controller::create))
    .route("/api/user/{id}", put(user_controller::update))
    .route("/api/user/{id}", delete(user_controller::delete))
    .route_layer(middleware::from_fn(auth_guard));

    let get_file_router = Router::new()
    .route("/api/files/user/image/{filename}", get(files_controller::get_user_image))
    .route("/api/files/product/image/{filename}", get(files_controller::get_product_image));

    let file_router = Router::new()
    .route("/api/files/user", post(files_controller::upload_user_image))
    .route("/api/files/user/delete/{filename}", delete(files_controller::delete_user_image))
    .route("/api/files/product", post(files_controller::upload_product_image))
    .route("/api/files/product/delete/{filename}", delete(files_controller::delete_product_image));

    let app_router = Router::new()
    .route("/api", get(|| async { "Hello World" }))
    .merge(category_router)
    .merge(user_router)
    .merge(login_router)
    .merge(auth_router)
    .merge(product_router)
    .merge(get_file_router)
    .merge(file_router)
    .layer(cors)
    .with_state(db);


    axum::serve(listener, app_router).await.expect("Error while serving the server.");
}
