use axum::{
    extract::{
        multipart::MultipartRejection,
        rejection::{FormRejection, QueryRejection},
    },
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_extra::extract::WithRejection;
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

pub type WithAppRejection<E> = WithRejection<E, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("I couldn't access the database! :(")]
    Database(#[from] sqlx::Error),
    #[error("Something weird went wrong :(")]
    MultipartError(#[from] axum::extract::multipart::MultipartError),
    #[error("I couldn't find the page you're looking for! :(")]
    NotFound,
    #[error("Please fill out all the fields!")]
    Form(#[from] FormRejection),
    #[error("Something weird went wrong :(")]
    MultipartRejection(#[from] MultipartRejection),
    #[error("Please change the following fields :3\n{0}")]
    ValidateForm(#[from] axum_valid::ValidRejection<FormRejection>),
    #[error("Please change the following fields :3\n{0}")]
    ValidateQuery(#[from] axum_valid::ValidRejection<QueryRejection>),
}

impl AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::Database(_) => StatusCode::SERVICE_UNAVAILABLE,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Form(_)
            | AppError::MultipartError(_)
            | AppError::MultipartRejection(_)
            | AppError::ValidateForm(_)
            | AppError::ValidateQuery(_) => StatusCode::BAD_REQUEST,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.status_code(), self.to_string()).into_response()
    }
}
