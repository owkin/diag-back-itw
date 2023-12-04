use crate::app::model::{EncodedImage, Job, JobRequest};
use crate::app::processing::{encode_buffer_rgb, process};
use crate::app::state::AppState;
use crate::error::AppError;
use anyhow::anyhow;
use axum::extract::{DefaultBodyLimit, Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use axum_typed_multipart::TypedMultipart;
use std::ops::Deref;
use tower_http::limit::RequestBodyLimitLayer;
use uuid::Uuid;

pub fn app() -> Router {
    let state = AppState::default();

    let app = Router::new()
        .route("/job", post(handle_job))
        .route("/job", get(get_jobs))
        .route("/job/:id", get(get_job))
        .route("/job/result/:id", get(get_result))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(25 * 1024 * 1024 /* 25mb */))
        .with_state(state);

    app
}

#[axum::debug_handler]
async fn get_job(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Job>, AppError> {
    let database = state.database.read().unwrap();
    let job: Job = database
        .get(&id)
        .map(|job| job.clone().into())
        .ok_or_else(|| AppError(anyhow!("Job not found".to_string())))?;

    Ok(Json(job))
}

#[axum::debug_handler]
async fn get_jobs(State(state): State<AppState>) -> Result<Json<Vec<Job>>, AppError> {
    let database = state.database.read().unwrap();
    let jobs: Vec<Job> = database.values().map(|job| job.clone().into()).collect();

    Ok(Json(jobs))
}

#[axum::debug_handler]
async fn get_result(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<EncodedImage, AppError> {
    let database = state.database.read().unwrap();
    let job = database
        .get(&id)
        .ok_or_else(|| AppError(anyhow!("Job not found".to_string())))?;

    let result = job.result.as_ref().ok_or_else(|| AppError(anyhow!("Result Image not found".to_string())))?;

    let encoded_result = encode_buffer_rgb(&result)?;

    Ok(encoded_result)
}

#[axum::debug_handler]
async fn handle_job(
    State(state): State<AppState>,
    data: TypedMultipart<JobRequest>,
) -> Result<Json<Uuid>, AppError> {
    let id = Uuid::new_v4();
    tokio::task::spawn_blocking(move || {
        process(id, data.deref(), state.database.clone());
    });

    Ok(Json(id))
}
