#[derive(Debug)]
pub enum ProcessError {
    FetchError { message: String },
    DataError { message: String },
    InternalError { message: String }, // Those will be hidden
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
