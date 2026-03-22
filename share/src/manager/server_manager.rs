use axum::{
    extract::{Path as AxumPath, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use labeldroid_types::config::Config;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::{
    path::{Path, PathBuf},
    sync::atomic::AtomicU64,
};
use tower_http::cors::{Any, CorsLayer};

use crate::{config::ConfigManager, utils::bind_available_port};

pub struct ServerManager {
    port: u16,
    app: Arc<Mutex<axum::Router>>,
}

#[derive(Clone)]
struct AppState {
    workspace_dir: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Shape {
    label: String,
    points: Vec<Vec<f64>>,
    group_id: Option<i32>,
    shape_type: String,
    flags: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct LabelmeData {
    version: String,
    flags: serde_json::Value,
    shapes: Vec<Shape>,
    image_path: String,
    image_data: Option<String>,
    image_height: u32,
    image_width: u32,
}

#[derive(Serialize)]
struct ImageInfo {
    name: String,
    has_annotation: bool,
}

pub static SERVER_PORT: AtomicU64 = AtomicU64::new(3480);

impl ServerManager {
    pub fn new(workspace_dir: PathBuf, port: u16) -> Self {
        let state = AppState { workspace_dir };

        // CORS
        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);

        let app = Router::new()
            .route("/api/images", get(list_images))
            .route("/api/images/{name}", get(get_image))
            .route(
                "/api/annotations/{name}",
                get(get_annotation)
                    .post(save_annotation)
                    .delete(delete_annotation),
            )
            .route("/api/config", get(get_config).post(save_config))
            .layer(cors)
            .with_state(state);

        Self {
            port,
            app: Arc::new(Mutex::new(app)),
        }
    }

    pub fn start(&self) {
        let app = Arc::clone(&self.app);
        let port = self.port;
        let _ = std::thread::spawn(move || {
            if let Ok(rt) = tokio::runtime::Runtime::new() {
                rt.block_on(async {
                    let app = app.lock().unwrap().clone();
                    let (listener, current_port) = bind_available_port(port).await.unwrap();
                    SERVER_PORT.store(current_port as u64, std::sync::atomic::Ordering::Relaxed);
                    println!("Listening on port {}", current_port);
                    axum::serve(listener, app).await.unwrap();
                });
            }
        });
    }
}

async fn list_images(State(state): State<AppState>) -> Json<Vec<ImageInfo>> {
    // log::info!("Using workspace directory: {:?}", &state.workspace_dir);
    let mut images = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&state.workspace_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    let ext_lower = ext.to_lowercase();
                    if ["jpg", "jpeg", "png", "bmp"].contains(&ext_lower.as_str()) {
                        let name = path
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string();
                        let stem = path.file_stem().unwrap_or_default().to_string_lossy();
                        let json_path = state.workspace_dir.join(format!("{}.json", stem));

                        images.push(ImageInfo {
                            name,
                            has_annotation: json_path.exists(),
                        });
                    }
                }
            }
        }
    }
    Json(images)
}

async fn get_image(
    State(state): State<AppState>,
    AxumPath(name): AxumPath<String>,
) -> impl IntoResponse {
    let path = state.workspace_dir.join(&name);
    if path.exists() && path.is_file() {
        if let Ok(bytes) = std::fs::read(path) {
            return (StatusCode::OK, bytes).into_response();
        }
    }
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({ "ok": false, "result": "Image not found" })),
    )
        .into_response()
}

async fn get_annotation(
    State(state): State<AppState>,
    AxumPath(name): AxumPath<String>,
) -> impl IntoResponse {
    let path = Path::new(&name);
    let stem = path.file_stem().unwrap_or_default().to_string_lossy();
    let json_path = state.workspace_dir.join(format!("{}.json", stem));

    if json_path.exists() {
        if let Ok(data) = std::fs::read_to_string(json_path) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&data) {
                return (StatusCode::OK, Json(json)).into_response();
            }
        }
    }
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({ "ok": false, "result": "Annotation not found" })),
    )
        .into_response()
}

async fn save_annotation(
    State(state): State<AppState>,
    AxumPath(name): AxumPath<String>,
    Json(payload): Json<LabelmeData>,
) -> impl IntoResponse {
    let path = Path::new(&name);
    let stem = path.file_stem().unwrap_or_default().to_string_lossy();
    let json_path = state.workspace_dir.join(format!("{}.json", stem));

    match std::fs::File::create(json_path) {
        Ok(file) => {
            if let Err(e) = serde_json::to_writer_pretty(file, &payload) {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({ "ok": false, "result": format!("Failed to serialize: {}", e) })),
                )
                    .into_response();
            }
            (StatusCode::OK, Json(serde_json::json!({ "ok": true, "result": "Saved" }))).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "ok": false, "result": format!("Failed to create file: {}", e) })),
        )
            .into_response(),
    }
}

async fn delete_annotation(
    State(state): State<AppState>,
    AxumPath(name): AxumPath<String>,
) -> impl IntoResponse {
    let path = Path::new(&name);
    let stem = path.file_stem().unwrap_or_default().to_string_lossy();
    let json_path = state.workspace_dir.join(format!("{}.json", stem));

    if !json_path.exists() {
        return (
            StatusCode::OK,
            Json(serde_json::json!({ "ok": true, "result": "Already deleted or not found" })),
        )
            .into_response();
    }

    match std::fs::remove_file(json_path) {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({ "ok": true, "result": "Deleted" })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "ok": false, "result": format!("Failed to delete file: {}", e) })),
        )
            .into_response(),
    }
}

async fn get_config() -> impl IntoResponse {
    let config = ConfigManager::get_config();
    (StatusCode::OK, Json(config)).into_response()
}

async fn save_config(Json(payload): Json<Config>) -> impl IntoResponse {
    ConfigManager::save_config(&payload);
    (
        StatusCode::OK,
        Json(serde_json::json!({ "ok": true, "result": "Config saved" })),
    )
        .into_response()
        .into_response()
}
