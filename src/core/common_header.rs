use reqwest::header::{HeaderMap, HeaderValue, COOKIE, USER_AGENT, ORIGIN, REFERER};
use std::env;

pub fn create_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    let token = env::var("XQ_A_TOKEN").unwrap_or_else(|_| String::from("default_token"));
    let r_token = env::var("XQ_R_TOKEN").unwrap_or_else(|_| String::from("default_r_token"));
    let cookie_value = format!("xq_a_token={};xq_r_token={};", token, r_token);
    headers.insert(COOKIE, HeaderValue::from_str(&cookie_value).unwrap());
    headers.insert(USER_AGENT, HeaderValue::from_str("Mozilla/5.0").unwrap());
    headers.insert(ORIGIN, HeaderValue::from_str("https://xueqiu.com").unwrap());
    headers.insert(REFERER, HeaderValue::from_str("https://xueqiu.com").unwrap());
    headers.insert("X-Requested-With", HeaderValue::from_str("XMLHttpRequest").unwrap());
    headers
}