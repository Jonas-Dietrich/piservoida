use std::process::Command;
use axum::{response::{IntoResponse, Response}, routing::{get, post}, Json, Router};

use serde_json::json;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let app = Router::new()
        .route("/start", post(startserver))
        .route("/power", get(is_server_powered_on))
        .route("/startmc", post(start_mc))
        .route("/stopmc", post(stop_mc))
        .route("/mcstatus", get(get_kurbling));

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

async fn start_mc() -> Response {
    let commandpath = "/home/hawara/starter.sh";

    let res = Command::new(commandpath)
        .output()
        .expect("Failed to execute!");

    let gyatt = String::from_utf8(res.stdout).unwrap();
    println!("started: {gyatt}");

    (
        [("Access-Control-Allow-Origin", "*")],
        Json(json!({"message" : "oida"}))
    ).into_response()
}

async fn stop_mc() -> Response {
    let commandpath = "/home/hawara/ender.sh";

    let res = Command::new(commandpath)
        .output()
        .expect("Failed to execute!");

    let gyatt = String::from_utf8(res.stdout).unwrap();
    println!("stopped: {gyatt}");

    (
        [("Access-Control-Allow-Origin", "*")],
        Json(json!({"message" : "na hawi"}))
    ).into_response()
}

async fn get_kurbling() -> Response {
    let commandpath = "/home/hawara/checker.sh";

    let res = Command::new(commandpath)
        .output()
        .expect("Failed to execute!");

    let gyatt = String::from_utf8(res.stdout).unwrap();
    let status = gyatt.contains("lock");

    println!("status: {status}");

    (
        [("Access-Control-Allow-Origin", "*")],
        Json(json!({"on" : status}))
    ).into_response()
}
