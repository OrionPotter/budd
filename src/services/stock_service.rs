use reqwest::Client;
use serde_json::Value;
use crate::errors::error::ApiError;
use crate::core::common_header::create_headers;
use log::info;
use chrono::{Utc};

async fn get_data_from_url(url: &str, client: &Client) -> Result<Value, ApiError> {
    info!("🚀 发起请求 → URL: {}", url);
    let start_time = std::time::Instant::now();
    
    let response = client
        .get(url)
        .headers(create_headers())
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .send()
        .await?;
    
    info!("✅ 请求耗时: {:.2}ms", start_time.elapsed().as_millis());
    
    if !response.status().is_success() {
        return Err(ApiError::ServiceError(format!(
            "请求失败，状态码: {}",
            response.status()
        )));
    }

    let body = response.text().await?;
    let json_data: Value = serde_json::from_str(&body)?;
    info!("✅ 返回体: {}", json_data);
    Ok(json_data)
}

// 获取股票数据
pub async fn get_stock_data(symbol: String, client: Client) -> Result<Value, ApiError> {
    let url = format!("https://stock.xueqiu.com/v5/stock/quote.json?symbol={}", symbol);
    let data = get_data_from_url(&url, &client).await?;
    data.get("data")
        .and_then(|d| d.get("quote"))
        .cloned()
        .ok_or_else(|| ApiError::NotFound("quote data not found".to_string()))
}

// 获取公司数据
pub async fn get_company_data(symbol: String, client: Client) -> Result<Value, ApiError> {
    let url = format!("https://stock.xueqiu.com/v5/stock/f10/cn/company.json?symbol={}", symbol);
    let data = get_data_from_url(&url, &client).await?;
    data.get("data")
        .and_then(|d| d.get("company"))
        .cloned()
        .ok_or_else(|| ApiError::NotFound("company data not found".to_string()))
}

// 获取十大股东
pub async fn get_top_holders(symbol: String, client: Client) -> Result<Value, ApiError> {
    let url = format!("https://stock.xueqiu.com/v5/stock/f10/cn/top_holders.json?symbol={}", symbol);
    let data = get_data_from_url(&url, &client).await?;
    data.get("data")
        .and_then(|d| d.get("items"))
        .cloned()
        .ok_or_else(|| ApiError::NotFound("top holders data not found".to_string()))
}

// 获取股东人数
pub async fn get_holders_nums(symbol: String, client: Client) -> Result<Value, ApiError> {
    let url = format!("https://stock.xueqiu.com/v5/stock/f10/cn/holders.json?symbol={}&extend=true", symbol);
    let data = get_data_from_url(&url, &client).await?;
    data.get("data")
        .and_then(|d| d.get("items"))
        .cloned()
        .ok_or_else(|| ApiError::NotFound("holders data not found".to_string()))
}

// 获取分红数据
pub async fn get_bonus(symbol: String, client: Client) -> Result<Value, ApiError> {
    let url = format!("https://stock.xueqiu.com/v5/stock/f10/cn/bonus.json?symbol={}", symbol);
    let data = get_data_from_url(&url, &client).await?;
    data.get("data")
        .and_then(|d| d.get("items"))
        .cloned()
        .ok_or_else(|| ApiError::NotFound("bonus data not found".to_string()))
}

// 获取主要指标
pub async fn get_indicator(symbol: String, client: Client) -> Result<Value, ApiError> {
    let url = format!("https://stock.xueqiu.com/v5/stock/finance/cn/indicator.json?symbol={}&type=all&is_detail=true&count=5", symbol);
    let data = get_data_from_url(&url, &client).await?;
    data.get("data")
        .and_then(|d| d.get("list"))
        .cloned()
        .ok_or_else(|| ApiError::NotFound("indicator data not found".to_string()))
}

// 获取利润表
pub async fn get_income(symbol: String, client: Client) -> Result<Value, ApiError> {
    let url = format!("https://stock.xueqiu.com/v5/stock/finance/cn/income.json?symbol={}&type=all&is_detail=true&count=5", symbol);
    let data = get_data_from_url(&url, &client).await?;
    data.get("data")
        .and_then(|d| d.get("list"))
        .cloned()
        .ok_or_else(|| ApiError::NotFound("income data not found".to_string()))
}

// 获取资产负债表
pub async fn get_balance(symbol: String, client: Client) -> Result<Value, ApiError> {
    let url = format!("https://stock.xueqiu.com/v5/stock/finance/cn/balance.json?symbol={}&type=all&is_detail=true&count=5", symbol);
    let data = get_data_from_url(&url, &client).await?;
    data.get("data")
        .and_then(|d| d.get("list"))
        .cloned()
        .ok_or_else(|| ApiError::NotFound("balance data not found".to_string()))
}

// 获取现金流量表
pub async fn get_cash_flow(symbol: String, client: Client) -> Result<Value, ApiError> {
    let url = format!("https://stock.xueqiu.com/v5/stock/finance/cn/cash_flow.json?symbol={}&type=all&is_detail=true&count=5", symbol);
    let data = get_data_from_url(&url, &client).await?;
    data.get("data")
        .and_then(|d| d.get("list"))
        .cloned()
        .ok_or_else(|| ApiError::NotFound("cash flow data not found".to_string()))
}

// 获取分时图
pub async fn get_minute_line(symbol: String, client: Client) -> Result<Value, ApiError> {
    let url = format!("https://stock.xueqiu.com/v5/stock/chart/minute.json?symbol={}&period=1d", symbol);
    let data = get_data_from_url(&url, &client).await?;
    data.get("data")
        .and_then(|d| d.get("items"))
        .cloned()
        .ok_or_else(|| ApiError::NotFound("minute line data not found".to_string()))
}

// 获取股票实时交易分笔数据
pub async fn get_trade(symbol: String, client: Client) -> Result<Value, ApiError> {
    let url = format!("https://stock.xueqiu.com/v5/stock/history/trade.json?symbol={}&count=10", symbol);
    let data = get_data_from_url(&url, &client).await?;
    data.get("data")
        .and_then(|d| d.get("items"))
        .cloned()
        .ok_or_else(|| ApiError::NotFound("trade data not found".to_string()))
}

// 获取盘口数据
pub async fn get_pankou(symbol: String, client: Client) -> Result<Value, ApiError> {
    let url = format!("https://stock.xueqiu.com/v5/stock/realtime/pankou.json?symbol={}", symbol);
    let data = get_data_from_url(&url, &client).await?;
    data.get("data")
        .cloned()
        .ok_or_else(|| ApiError::NotFound("trade data not found".to_string()))
}

const VALID_PERIODS: [&str; 11] = [
    "day", "week", "month", "quarter", "year",
    "120m", "60m", "30m", "15m", "5m", "1m"
];

// 获取K线数据
pub async fn get_k_line(symbol: String, client: Client, interval: String) -> Result<Value, ApiError> {
    // 验证period参数是否合法
    if !VALID_PERIODS.contains(&interval .as_str()) {
        return Err(ApiError::NotFound("interval  data not found".to_string()));
    }
    let begin_timestamp = Utc::now().timestamp_millis();
    let url = format!("https://stock.xueqiu.com/v5/stock/chart/kline.json?symbol={}&begin={}&period={}&type=before&count=-284&indicator=kline,pe,pb,ps,pcf,market_capital,agt,ggt,balance", symbol, begin_timestamp, interval);
    let data = get_data_from_url(&url, &client).await?;
    data.get("data")
        .cloned()
        .ok_or_else(|| ApiError::NotFound("trade data not found".to_string()))
}





