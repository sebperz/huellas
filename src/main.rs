mod export;
mod pdf;

use axum::{
    extract::Multipart,
    http::{header, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Router,
};
use pdf::PdfResult;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};

struct AppState {
    last_results: Mutex<Vec<PdfResult>>,
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        last_results: Mutex::new(Vec::new()),
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(index))
        .route("/api/analyze", post(analyze))
        .route("/api/export/csv", get(export_csv))
        .route("/api/export/xlsx", get(export_xlsx))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    let addr = listener.local_addr().unwrap();
    let url = format!("http://{}", addr);

    println!("Huellas iniciado en {}", url);

    let _ = webbrowser::open(&url);

    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Html<&'static str> {
    Html(INDEX_HTML)
}

async fn analyze(
    state: axum::extract::State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut files: Vec<(String, String, Vec<u8>)> = Vec::new();

    while let Ok(Some(field)) = multipart.next_field().await {
        let filename = field
            .file_name()
            .unwrap_or("unknown.pdf")
            .to_string();

        let relative_path = field
            .name()
            .unwrap_or("")
            .to_string();

        let data = field.bytes().await.map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                format!("Error leyendo archivo: {}", e),
            )
        })?;

        files.push((filename, relative_path, data.to_vec()));
    }

    if files.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "No se enviaron archivos".to_string()));
    }

    let pdf_files: Vec<_> = files
        .into_iter()
        .filter(|(name, _, _)| {
            name.to_lowercase().ends_with(".pdf")
        })
        .collect();

    if pdf_files.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "No se encontraron archivos PDF".to_string()));
    }

    let mut results: Vec<PdfResult> = Vec::new();

    for (filename, relative_path, data) in &pdf_files {
        let result = pdf::analyze_pdf(data, filename, relative_path);
        results.push(result);
    }

    let json = serde_json::to_string(&results).map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    *state.last_results.lock().await = results;

    Ok(axum::response::Json(
        serde_json::from_str::<serde_json::Value>(&json).unwrap(),
    ))
}

async fn export_csv(
    state: axum::extract::State<Arc<AppState>>,
) -> Result<Response, (StatusCode, String)> {
    let results = state.last_results.lock().await;

    if results.is_empty() {
        return Err((StatusCode::NOT_FOUND, "No hay resultados para exportar".to_string()));
    }

    let csv_content = export::generate_csv(&results);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(
            header::CONTENT_TYPE,
            "text/csv; charset=utf-8",
        )
        .header(
            header::CONTENT_DISPOSITION,
            "attachment; filename=\"resultados.csv\"",
        )
        .body(csv_content.into())
        .unwrap())
}

async fn export_xlsx(
    state: axum::extract::State<Arc<AppState>>,
) -> Result<Response, (StatusCode, String)> {
    let results = state.last_results.lock().await;

    if results.is_empty() {
        return Err((StatusCode::NOT_FOUND, "No hay resultados para exportar".to_string()));
    }

    let xlsx_bytes = export::generate_xlsx(&results).map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(
            header::CONTENT_TYPE,
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        )
        .header(
            header::CONTENT_DISPOSITION,
            "attachment; filename=\"resultados.xlsx\"",
        )
        .body(xlsx_bytes.into())
        .unwrap())
}

const INDEX_HTML: &str = include_str!("index.html");
