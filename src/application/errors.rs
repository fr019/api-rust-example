use crate::handlers::user;
use axum::{
    extract::rejection::FormRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;
#[derive(Debug, Error)]
pub enum ApiError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumFormRejection(#[from] FormRejection),

    #[error(transparent)]
    UserError(#[from] user::UserError),
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
        };
        res.into_response()
    }
}
