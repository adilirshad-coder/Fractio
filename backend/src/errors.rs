use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use uuid::Uuid;
#[derive(Debug, thiserror::Error)]
pub enum ApiError { #[error("not implemented")] NotImplemented, #[error("unauthorized")] Unauthorized, #[error("forbidden")] Forbidden, #[error("invalid request: {0}")] Validation(String), #[error("internal error")] Internal(#[from] anyhow::Error) }
#[derive(Serialize)] struct Body { error: ErrorBody, request_id: String }
#[derive(Serialize)] struct ErrorBody { code: &'static str, message: String }
impl IntoResponse for ApiError {
 fn into_response(self) -> axum::response::Response {
  let (status, code, message) = match self { Self::NotImplemented => (StatusCode::NOT_IMPLEMENTED,"NOT_IMPLEMENTED","Business behavior is not implemented".into()), Self::Unauthorized => (StatusCode::UNAUTHORIZED,"UNAUTHORIZED","Authentication required".into()), Self::Forbidden => (StatusCode::FORBIDDEN,"FORBIDDEN","Insufficient permissions".into()), Self::Validation(m) => (StatusCode::BAD_REQUEST,"VALIDATION_ERROR",m), Self::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR,"INTERNAL_ERROR","Request could not be completed".into()) };
  (status, Json(Body { error: ErrorBody { code, message }, request_id: Uuid::new_v4().to_string() })).into_response()
 }
}
