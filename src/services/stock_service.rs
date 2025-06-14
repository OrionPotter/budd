use reqwest::{Client};
use serde_json::Value;
use crate::errors::error::{MyError};
use crate::core::common_header::create_headers;

async fn get_data_from_url(url: &str, client: &Client) -> Result<Value, MyError> {
    // 发送 HTTP 请求
    let response = client
        .get(url)
        .headers(create_headers())
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .send()
        .await;

    // 错误处理：检查请求是否成功
    let response = match response {
        Ok(res) if res.status().is_success() => res,
        Ok(res) => {
            // 如果响应失败，返回自定义错误
            return Err(MyError::OtherError(format!(
                "请求失败，状态码: {}",
                res.status()
            )));
        }
        Err(e) => {
            // 请求出错
            return Err(MyError::ReqwestError(e));
        }
    };

    // 解析响应的 JSON 数据
    let body = response.text().await.map_err(|e| MyError::ReqwestError(e))?;
    let json_data: Value = serde_json::from_str(&body).map_err(|e| MyError::JsonError(e))?;

    // 返回解析后的 JSON 数据
    Ok(json_data)
}

// 获取股票数据
pub async fn get_stock_data(symbol: &str, client: &Client) -> Result<Value, MyError> {
    let url = format!("https://stock.xueqiu.com/v5/stock/quote.json?symbol={}", symbol);
    get_data_from_url(&url, client).await
}

// 获取公司数据
pub async fn get_company_data(symbol: &str, client: &Client) -> Result<Value, MyError> {
    let url = format!("https://stock.xueqiu.com/v5/stock/f10/cn/company.json?symbol={}", symbol);
    get_data_from_url(&url, client).await
}
