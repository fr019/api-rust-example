use crate::handlers::users::UserError;
use axum::extract::rejection::{JsonRejection, QueryRejection};
use axum::http::header::InvalidHeaderValue;
use axum::{
    extract::rejection::FormRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error(transparent)]
    ValidationError(#[from] ValidationErrors),

    #[error(transparent)]
    AxumFormRejection(#[from] FormRejection),

    #[error(transparent)]
    UserError(#[from] UserError),

    #[error(transparent)]
    JsonParseError(#[from] JsonRejection),

    #[error(transparent)]
    QueryParseError(#[from] QueryRejection),

    #[error(transparent)]
    InvalidContentType(#[from] InvalidHeaderValue),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let res = match self {
            ApiError::ValidationError(_) => {
                let message = format!("Input validation error: [{self}]").replace('\n', ", ");
                (StatusCode::BAD_REQUEST, message)
            }
            ApiError::AxumFormRejection(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            ApiError::UserError(e) => e.prepare_api_error(),
            ApiError::JsonParseError(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            ApiError::QueryParseError(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            ApiError::InvalidContentType(_) => (StatusCode::BAD_REQUEST, self.to_string()),
        };
        res.into_response()
    }
}
