use axum::{
    extract::{Path as AxumPath, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tauri::Manager;
use tower_http::cors::{Any, CorsLayer};

pub struct FileManangerServer {
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

impl FileManangerServer {
    pub fn new(app_handle: tauri::AppHandle) -> Self {
        let workspace_dir = if let Ok(dir) = app_handle.path().download_dir() {
            dir
        } else {
            #[cfg(target_os = "android")]
            let fallback_dir = PathBuf::from("/storage/emulated/0/Download/labeldroid");

            #[cfg(not(target_os = "android"))]
            let fallback_dir = {
                let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
                current_dir.parent().unwrap_or(&current_dir).join("tmp")
            };

            fallback_dir
        };

        if !workspace_dir.exists() {
            std::fs::create_dir_all(&workspace_dir).unwrap_or(());
        }

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
                get(get_annotation).post(save_annotation),
            )
            .layer(cors)
            .with_state(state);

        Self {
            app: Arc::new(Mutex::new(app)),
        }
    }

    pub fn start(&self) {
        let app = Arc::clone(&self.app);
        let _ = std::thread::spawn(move || {
            if let Ok(rt) = tokio::runtime::Runtime::new() {
                rt.block_on(async {
                    let app = app.lock().unwrap().clone();
                    // Using IP and Port 3000
                    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
                    println!("Listening on port 3000");
                    axum::serve(listener, app).await.unwrap();
                });
            }
        });
    }
}

async fn list_images(State(state): State<AppState>) -> Json<Vec<ImageInfo>> {
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
    (StatusCode::NOT_FOUND, "Image not found").into_response()
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
    (StatusCode::NOT_FOUND, "Annotation not found").into_response()
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
                    format!("Failed to serialize: {}", e),
                )
                    .into_response();
            }
            (StatusCode::OK, "Saved").into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create file: {}", e),
        )
            .into_response(),
    }
}
