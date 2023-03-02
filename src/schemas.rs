use poem_openapi::payload::Json;
use poem_openapi::types::Example;
use poem_openapi::{ApiResponse, Object};

use crate::utils::capitalize_string;

/// Error schema
#[derive(Debug, Object, serde::Serialize)]
#[oai(example)]
pub struct ErrorSchema {
    /// Error message
    message: String,
}

impl ErrorSchema {
    pub fn new(s: String) -> Self {
        Self {
            message: capitalize_string(s),
        }
    }
}

impl Example for ErrorSchema {
    fn example() -> Self {
        ErrorSchema {
            message: "Something went wrong".to_string(),
        }
    }
}

/// Screenshot schema
#[derive(Debug, Object)]
#[oai(example)]
pub struct ScreenshotSchema {
    /// Screenshot data
    data: String,
}

impl ScreenshotSchema {
    pub fn new(data: String) -> Self {
        Self { data }
    }
}

impl Example for ScreenshotSchema {
    fn example() -> Self {
        ScreenshotSchema {
            data: "Example".to_string(),
        }
    }
}

#[derive(ApiResponse)]
pub enum GetScreenshotResponse {
    #[oai(status = 200)]
    Ok(Json<ScreenshotSchema>),
    #[oai(status = 500)]
    InternalServerError(Json<ErrorSchema>),
}

impl From<String> for GetScreenshotResponse {
    fn from(data: String) -> Self {
        Self::Ok(Json(ScreenshotSchema::new(data)))
    }
}
