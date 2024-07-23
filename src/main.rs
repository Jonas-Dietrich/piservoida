use std::process::Command;
use axum::{http::{header::ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue}, response::{IntoResponse, Response}, routing::{get, post}, Json, Router};

use serde_json::{Value, json};

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let app = Router::new()
        .route("/start", post(startserver))
        .route("/power", get(is_server_powered_on));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();

    println!("goodbye, world!")
}

async fn is_server_powered_on() -> Response {
    let res = Command::new("ping")
        .args(["192.168.0.75", "-c 1"])
        .output()
        .expect("Fehler");

    let rizz = String::from_utf8(res.stdout).unwrap();

    let var = rizz.contains("1 received");
    println!("on: {var}");

    (
        [("Access-Control-Allow-Origin", "*")],
        Json(json!({"message" : var}))
    ).into_response()

}

async fn startserver() -> Response {
    let commandpath = "/home/hawara/wakelappy.sh";

    let res = Command::new(commandpath)
        .output()
        .expect("Failed to execute!");

    let gyatt = String::from_utf8(res.stdout).unwrap();

    println!("sent a packet");

    (
        [("Access-Control-Allow-Origin", "*")],
        Json(json!({"message" : gyatt}))
    ).into_response()
} 
