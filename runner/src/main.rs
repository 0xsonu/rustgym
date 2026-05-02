use std::sync::Arc;

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod executor;
mod output;
mod sandbox;
mod syntest;

use executor::{DockerExecutor, WorkspaceFiles};
use output::parse_cargo_test_output;
use sandbox::SandboxConfig;

// --- Shared application state ---

struct AppState {
    executor: DockerExecutor,
}

// --- Request/Response types ---

#[derive(Debug, Deserialize)]
struct RunTestRequest {
    /// Base64-encoded user code (lib.rs)
    code_b64: String,
    /// Base64-encoded test code (tests/tests.rs)
    test_b64: String,
    /// Base64-encoded Cargo.toml
    cargo_toml_b64: String,
    /// Optional syntest AST validation rules
    syntest_rules: Option<Vec<String>>,
    /// Optional timeout override in seconds
    timeout_secs: Option<u64>,
}

#[derive(Debug, Serialize)]
struct TestResultItem {
    name: String,
    passed: bool,
    message: Option<String>,
}

#[derive(Debug, Serialize)]
struct RunTestResponse {
    status: String,
    stdout: String,
    stderr: String,
    tests: Vec<TestResultItem>,
    duration_ms: u64,
    memory_kb: u64,
}

#[derive(Debug, Deserialize)]
struct PlaygroundRequest {
    /// Base64-encoded user code
    code_b64: String,
}

#[derive(Debug, Serialize)]
struct PlaygroundResponse {
    status: String,
    stdout: String,
    stderr: String,
    duration_ms: u64,
}

// --- Handlers ---

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "service": "rustgym-runner"
    }))
}

async fn run_test(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RunTestRequest>,
) -> Json<Value> {
    // Decode base64 payloads
    let code = match BASE64.decode(&payload.code_b64) {
        Ok(bytes) => match String::from_utf8(bytes) {
            Ok(s) => s,
            Err(_) => {
                return Json(json!(RunTestResponse {
                    status: "compile_error".to_string(),
                    stdout: String::new(),
                    stderr: "Invalid UTF-8 in code payload".to_string(),
                    tests: vec![],
                    duration_ms: 0,
                    memory_kb: 0,
                }));
            }
        },
        Err(_) => {
            return Json(json!(RunTestResponse {
                status: "compile_error".to_string(),
                stdout: String::new(),
                stderr: "Invalid base64 in code_b64".to_string(),
                tests: vec![],
                duration_ms: 0,
                memory_kb: 0,
            }));
        }
    };

    let test_code = match BASE64.decode(&payload.test_b64) {
        Ok(bytes) => match String::from_utf8(bytes) {
            Ok(s) => s,
            Err(_) => {
                return Json(json!(RunTestResponse {
                    status: "compile_error".to_string(),
                    stdout: String::new(),
                    stderr: "Invalid UTF-8 in test payload".to_string(),
                    tests: vec![],
                    duration_ms: 0,
                    memory_kb: 0,
                }));
            }
        },
        Err(_) => {
            return Json(json!(RunTestResponse {
                status: "compile_error".to_string(),
                stdout: String::new(),
                stderr: "Invalid base64 in test_b64".to_string(),
                tests: vec![],
                duration_ms: 0,
                memory_kb: 0,
            }));
        }
    };

    let cargo_toml = match BASE64.decode(&payload.cargo_toml_b64) {
        Ok(bytes) => match String::from_utf8(bytes) {
            Ok(s) => s,
            Err(_) => {
                return Json(json!(RunTestResponse {
                    status: "compile_error".to_string(),
                    stdout: String::new(),
                    stderr: "Invalid UTF-8 in cargo_toml payload".to_string(),
                    tests: vec![],
                    duration_ms: 0,
                    memory_kb: 0,
                }));
            }
        },
        Err(_) => {
            return Json(json!(RunTestResponse {
                status: "compile_error".to_string(),
                stdout: String::new(),
                stderr: "Invalid base64 in cargo_toml_b64".to_string(),
                tests: vec![],
                duration_ms: 0,
                memory_kb: 0,
            }));
        }
    };

    // Run syntest AST validation if rules are provided
    if let Some(ref rules) = payload.syntest_rules {
        if !rules.is_empty() {
            match syntest::validate(&code, rules) {
                Ok(violations) if !violations.is_empty() => {
                    let messages: Vec<String> =
                        violations.iter().map(|v| v.message.clone()).collect();
                    return Json(json!(RunTestResponse {
                        status: "syntest_failed".to_string(),
                        stdout: String::new(),
                        stderr: messages.join("\n"),
                        tests: vec![],
                        duration_ms: 0,
                        memory_kb: 0,
                    }));
                }
                Err(parse_err) => {
                    return Json(json!(RunTestResponse {
                        status: "syntest_failed".to_string(),
                        stdout: String::new(),
                        stderr: parse_err,
                        tests: vec![],
                        duration_ms: 0,
                        memory_kb: 0,
                    }));
                }
                _ => {} // Validation passed, continue
            }
        }
    }

    // Build workspace files
    let files = WorkspaceFiles {
        cargo_toml,
        lib_rs: code,
        test_rs: Some(test_code),
        as_binary: false,
    };

    // Execute in container with optional timeout override
    let result = match state
        .executor
        .execute(
            files,
            vec![
                "cargo".to_string(),
                "test".to_string(),
                "--no-fail-fast".to_string(),
            ],
            payload.timeout_secs,
        )
        .await
    {
        Ok(r) => r,
        Err(e) => {
            tracing::error!(error = %e, "Docker execution failed");
            return Json(json!(RunTestResponse {
                status: "runtime_error".to_string(),
                stdout: String::new(),
                stderr: format!("Execution error: {e}"),
                tests: vec![],
                duration_ms: 0,
                memory_kb: 0,
            }));
        }
    };

    // Determine status from execution result
    let (tests, _summary) = parse_cargo_test_output(&result.stdout);
    let test_items: Vec<TestResultItem> = tests
        .into_iter()
        .map(|t| TestResultItem {
            name: t.name,
            passed: t.passed,
            message: t.message,
        })
        .collect();

    let status = if result.timed_out {
        "timeout".to_string()
    } else if let Some(exit_code) = result.exit_code {
        if exit_code == 0 && test_items.iter().all(|t| t.passed) {
            "passed".to_string()
        } else if exit_code == 0 || test_items.iter().any(|t| !t.passed) {
            "failed".to_string()
        } else if result.stderr.contains("error[E") {
            "compile_error".to_string()
        } else {
            "runtime_error".to_string()
        }
    } else {
        "runtime_error".to_string()
    };

    Json(json!(RunTestResponse {
        status,
        stdout: result.stdout,
        stderr: result.stderr,
        tests: test_items,
        duration_ms: result.duration_ms,
        memory_kb: 0,
    }))
}

async fn run_playground(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<PlaygroundRequest>,
) -> Json<Value> {
    // Decode base64 code
    let code = match BASE64.decode(&payload.code_b64) {
        Ok(bytes) => match String::from_utf8(bytes) {
            Ok(s) => s,
            Err(_) => {
                return Json(json!(PlaygroundResponse {
                    status: "compile_error".to_string(),
                    stdout: String::new(),
                    stderr: "Invalid UTF-8 in code payload".to_string(),
                    duration_ms: 0,
                }));
            }
        },
        Err(_) => {
            return Json(json!(PlaygroundResponse {
                status: "compile_error".to_string(),
                stdout: String::new(),
                stderr: "Invalid base64 in code_b64".to_string(),
                duration_ms: 0,
            }));
        }
    };

    // Create a default Cargo.toml for a binary project
    let cargo_toml = r#"[package]
name = "playground"
version = "0.1.0"
edition = "2021"
"#
    .to_string();

    // For playground, the code is a binary (main.rs)
    let files = WorkspaceFiles {
        cargo_toml,
        lib_rs: code,
        test_rs: None,
        as_binary: true,
    };

    // Execute with `cargo run`
    let result = match state
        .executor
        .execute(files, vec!["cargo".to_string(), "run".to_string()], None)
        .await
    {
        Ok(r) => r,
        Err(e) => {
            tracing::error!(error = %e, "Docker execution failed");
            return Json(json!(PlaygroundResponse {
                status: "runtime_error".to_string(),
                stdout: String::new(),
                stderr: format!("Execution error: {e}"),
                duration_ms: 0,
            }));
        }
    };

    // Determine status
    let status = if result.timed_out {
        "timeout".to_string()
    } else if let Some(exit_code) = result.exit_code {
        if exit_code == 0 {
            "success".to_string()
        } else if result.stderr.contains("error[E") {
            "compile_error".to_string()
        } else {
            "runtime_error".to_string()
        }
    } else {
        "runtime_error".to_string()
    };

    Json(json!(PlaygroundResponse {
        status,
        stdout: result.stdout,
        stderr: result.stderr,
        duration_ms: result.duration_ms,
    }))
}

#[tokio::main]
async fn main() {
    // Set up tracing subscriber
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create the Docker executor with default sandbox config
    let executor =
        DockerExecutor::new(SandboxConfig::default()).expect("Failed to connect to Docker daemon");

    let state = Arc::new(AppState { executor });

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/run/test", post(run_test))
        .route("/run/playground", post(run_playground))
        .with_state(state);

    let addr = "0.0.0.0:3001";
    tracing::info!("RustGym runner service starting on {}", addr);

    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");
    axum::serve(listener, app).await.expect("Server error");
}
