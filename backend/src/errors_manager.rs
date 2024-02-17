use actix_web::{http::StatusCode, web::Json, HttpResponse, ResponseError};
use serde::Serialize;

#[derive(Debug)]
pub enum ProcessError {
    FetchError { message: String },
    DataError { message: String }, // Maybe unused given that I still want to output results
    InternalError { message: String }, // Those will be hidden
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl ResponseError for ProcessError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ProcessError::FetchError { .. } => StatusCode::SERVICE_UNAVAILABLE,
            ProcessError::DataError { .. } => StatusCode::BAD_REQUEST,
            ProcessError::InternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorResponse {
            error: self.to_string(),
        })
    }
}

impl From<reqwest::Error> for ProcessError {
    fn from(e: reqwest::Error) -> Self {
        ProcessError::FetchError {
            message:
                "There was a problem fetching the data. Please check your internet connection."
                    .into(),
        }
    }
}

impl From<serde_json::Error> for ProcessError {
    fn from(e: serde_json::Error) -> Self {
        ProcessError::DataError {
            message: "There was a problem processing the data.".into(),
        }
    }
}

impl std::fmt::Display for ProcessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ProcessError::FetchError { ref message }
            | ProcessError::DataError { ref message }
            | ProcessError::InternalError { ref message } => write!(f, "{}", message),
        }
    }
}

impl std::error::Error for ProcessError {}
