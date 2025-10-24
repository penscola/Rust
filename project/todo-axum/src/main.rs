use std::sync::{Arc, Mutex};

use axum::{
    routing::{get, post},
    Router,
};
use crate::controller::create_todo::create_todo;

mod controller;
mod types;

use crate::types::Db;

#[tokio::main]
async fn main() {
    std::fs::create_dir("data");

    let connection = sqlite::open("data/todo.db").unwrap();
    let db: Db = Arc::new(Mutex::new(connection));
    
    let app = Router::new().route("/", get(|| async { "welcome to Nairobi, World!" }))
    .route("/todo", post(create_todo))
    .with_state(db);

    // run our app with hyper, listening globally on port 8000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}