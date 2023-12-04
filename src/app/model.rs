use axum::body::Body;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_typed_multipart::{TryFromField, TryFromMultipart};
use serde::{Deserialize, Serialize};
use axum::response::Response;

#[derive(TryFromMultipart, Debug)]
pub struct JobRequest {
    #[form_data(limit = "25MiB")]
    pub image: axum::body::Bytes,
    pub filter: FilterType,
    pub sigma: f32,
}

#[derive(TryFromField, Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum FilterType {
    Blurring,
    UnSharpening,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Processing,
    Error,
    Done,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Job {
    pub id: uuid::Uuid,
    pub filter: FilterType,
    pub status: Status,
    pub start_time: chrono::NaiveDateTime,
    pub end_time: Option<chrono::NaiveDateTime>,
}

#[derive(Debug)]
pub struct EncodedImage {
    pub data: Vec<u8>,
}

impl IntoResponse for EncodedImage {
    fn into_response(self) -> Response {
        let content_type = axum::http::header::HeaderValue::from_static("image/png");

        Response::builder()
            .status(StatusCode::from_u16(200).unwrap())
            .header(axum::http::header::CONTENT_TYPE, content_type)
            .body(Body::from(self.data))
            .unwrap()
    }
}