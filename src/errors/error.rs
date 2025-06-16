use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;
use crate::core::response::ApiResponse;

#[derive(Debug, Serialize)]
pub enum ApiError {
    ReqwestError(String),
    JsonError(String),
    NotFound(String),
    ServiceError(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::ReqwestError(err) => write!(f, "Request error: {}", err),
            ApiError::JsonError(err) => write!(f, "JSON parse error: {}", err),
            ApiError::NotFound(err) => write!(f, "Resource not found: {}", err),
            ApiError::ServiceError(err) => write!(f, "Service error: {}", err),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let status = match self {
            ApiError::NotFound(_) => actix_web::http::StatusCode::NOT_FOUND,
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        };

        HttpResponse::build(status).json(ApiResponse::<()>::error(&self.to_string()))
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(error: reqwest::Error) -> Self {
        ApiError::ReqwestError(error.to_string())
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(error: serde_json::Error) -> Self {
        ApiError::JsonError(error.to_string())
    }
}