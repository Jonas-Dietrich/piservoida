use std::process::Command;
use axum::{routing::{get, post}, Json, Router};

use serde_json::{Value, json};

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let oid = startserver().await;

    println!("oiad {}", oid.0);

    let app = Router::new()
        .route("/", get(oida))
        .route("/start", post(startserver));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();

    println!("goodbye, world!")
}

async fn oida() -> &'static str {
    "Oida"
}

async fn startserver() -> Json<Value> {
    let commandpath = "/home/jonas/oida.sh";
    println!("oid {commandpath}");

    let res = Command::new(commandpath)
        .output()
        .expect("Failed to execute!");

    let gyatt = String::from_utf8(res.stdout).unwrap();

    println!("oida: {gyatt}");

    Json(json!({"message" : gyatt}))
} 
