use reqwest::header::{HeaderMap, HeaderValue, COOKIE};
use std::env;

pub fn create_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    let token = env::var("XQ_A_TOKEN").unwrap_or_else(|_| String::from("default_token"));
    let r_token = env::var("XQ_R_TOKEN").unwrap_or_else(|_| String::from("default_r_token"));
    let cookie_value = format!("xq_a_token={};xq_r_token={};", token, r_token);
    headers.insert(COOKIE, HeaderValue::from_str(&cookie_value).unwrap());
    headers
}