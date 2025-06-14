use std::fmt;
use reqwest::{StatusCode, Url};


#[derive(Debug)]
#[allow(dead_code)]
pub enum MyError {
    ReqwestError(reqwest::Error),
    JsonError(serde_json::Error),
    HeaderError(String),
    OtherError(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MyError::ReqwestError(ref err) => write!(f, "Request error: {}", err),
            MyError::JsonError(ref err) => write!(f, "JSON parse error: {}", err),
            MyError::HeaderError(ref err) => write!(f, "Header error: {}", err),
            MyError::OtherError(ref err) => write!(f, "Other error: {}", err),
        }
    }
}

impl From<reqwest::Error> for MyError {
    fn from(error: reqwest::Error) -> Self {
        MyError::ReqwestError(error)
    }
}

impl From<serde_json::Error> for MyError {
    fn from(error: serde_json::Error) -> Self {
        MyError::JsonError(error)
    }
}

impl From<String> for MyError {
    fn from(error: String) -> Self {
        MyError::OtherError(error)
    }
}

#[allow(dead_code)]
pub fn status_code(url: Url, status: StatusCode) -> MyError {
    MyError::OtherError(format!("HTTP request failed for URL {} with status code {}", url, status))
}
