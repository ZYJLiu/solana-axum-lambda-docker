// cargo build --release
// cargo watch -q -c -w src/ -x "run"

// cargo lambda watch
// http://localhost:9000
//
// cargo lambda build --release --arm64
use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use lambda_http::{run, tracing, Error};
use serde::Deserialize;
use serde_json::{json, Value};
use std::net::{Ipv4Addr, SocketAddr};
use std::process::Command;
use tokio::net::TcpListener;

mod error;
mod program;
mod routes;
use self::routes::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // required to enable CloudWatch error logging by the runtime
    tracing::init_default_subscriber();

    print!("hello");

    let app = Router::new()
        .route("/build", post(build))
        .route("/deploy/:uuid", get(deploy))
        .route("/hello", get(hello))
        .route("/", get(get_cargo_sbf_build_help))
        .route("/", post(get_solana_account));

    if std::env::var("AWS_LAMBDA_RUNTIME_API").is_ok() {
        run(app).await
    } else {
        let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080u16));
        let listener = TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;
        Ok(())
    }

    // run(app).await

    // let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080u16));
    // let listener = TcpListener::bind(addr).await.unwrap();
    // axum::serve(listener, app).await.unwrap();
}

async fn hello() -> impl IntoResponse {
    "Hello World!"
}

async fn get_cargo_sbf_build_help() -> Result<Json<Value>, ApiError> {
    let output = Command::new("cargo-build-sbf")
        .arg("--help")
        .output()
        .map_err(|e| ApiError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: json!({"error": format!("Failed to execute command: {}", e)}),
        })?;

    if !output.status.success() {
        return Err(ApiError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: json!({"error": "Command did not run successfully"}),
        });
    }

    let help_output = String::from_utf8(output.stdout).map_err(|e| ApiError {
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        message: json!({"error": format!("Failed to parse output: {}", e)}),
    })?;

    let mut help_output_map = serde_json::Map::new();
    for line in help_output.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        if parts.len() == 2 {
            help_output_map.insert(parts[0].to_string(), json!(parts[1]));
        }
    }

    let formatted_output: Vec<String> = help_output.lines().map(|line| line.to_string()).collect();

    Ok(Json(json!({
        "cargo sbf build --help": formatted_output
    })))
}

#[derive(Deserialize)]
struct SolanaAccountRequest {
    address: String,
}

async fn get_solana_account(
    Json(payload): Json<SolanaAccountRequest>,
) -> Result<Json<Value>, ApiError> {
    let output = Command::new("solana")
        .arg("account")
        .arg(&payload.address)
        .output()
        .map_err(|e| ApiError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: json!({"error": format!("Failed to execute command: {}", e)}),
        })?;

    if !output.status.success() {
        return Err(ApiError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: json!({"error": "Command did not run successfully"}),
        });
    }

    let account_info = String::from_utf8(output.stdout).map_err(|e| ApiError {
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        message: json!({"error": format!("Failed to parse output: {}", e)}),
    })?;

    let mut account_info_map = serde_json::Map::new();
    for line in account_info.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        if parts.len() == 2 {
            account_info_map.insert(parts[0].to_string(), json!(parts[1]));
        }
    }

    Ok(Json(Value::Object(account_info_map)))
}

#[derive(Debug)]
struct ApiError {
    status_code: StatusCode,
    message: Value,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.status_code, Json(self.message)).into_response()
    }
}
