use actix_web::{web, App, HttpServer, HttpResponse};
use reqwest::Client;
use crate::services::stock_service::{get_balance, get_bonus, get_cash_flow, get_company_data, get_holders_nums, get_income, get_indicator, get_stock_data, get_top_holders};

mod core;
mod services;
mod errors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info) // 默认日志级别
        .init();
    // 启动 HTTP 服务器并绑定到指定地址
    HttpServer::new(|| {
        App::new()
            .route("/stock/{symbol}", web::get().to(stock_handler)) 
            .route("/company/{symbol}", web::get().to(company_handler))
            .route("/top_holders/{symbol}", web::get().to(top_holder_handler))
            .route("/holders/{symbol}", web::get().to(holders_num_handler))
            .route("/bonus/{symbol}", web::get().to(bonus_handler))
            .route("/indicator/{symbol}", web::get().to(indicator_handler))
            .route("/income/{symbol}", web::get().to(income_handler))
            .route("/balance/{symbol}", web::get().to(balance_handler))
            .route("/cash_flow/{symbol}", web::get().to(cash_flow_handler))
    })
        .bind("127.0.0.1:8080")? // 监听地址和端口
        .run()
        .await
}

// 处理请求的异步函数
async fn stock_handler(path: web::Path<(String,)>) -> HttpResponse {
    let symbol = &path.0;  // 解构 path 中的元组，直接获取 symbol 参数

    // 创建 reqwest 客户端
    let client = Client::new();

    // 获取股票数据
    match get_stock_data(symbol, &client).await {
        Ok(data) => {
            // 返回结果作为 JSON 响应
            if let Some(quote) = data.get("data").and_then(|data| data.get("quote")) {
                HttpResponse::Ok().json(quote) // 返回 JSON 格式的 quote 部分
            } else {
                HttpResponse::NotFound().body("未找到 quote 字段") // 返回 404 如果没有找到 quote
            }
        }
        Err(err) => {
            eprintln!("请求失败: {}", err);
            HttpResponse::InternalServerError().body(format!("请求失败: {}", err)) // 返回 500 错误
        }
    }
}


async fn company_handler(path: web::Path<(String,)>) -> HttpResponse {
    let symbol = &path.0;  // 解构 path 中的元组，直接获取 symbol 参数

    // 创建 reqwest 客户端
    let client = Client::new();

    // 获取股票数据
    match get_company_data(symbol, &client).await {
        Ok(data) => {
            // 返回结果作为 JSON 响应
            if let Some(company) = data.get("data").and_then(|data| data.get("company")) {
                HttpResponse::Ok().json(company) 
            } else {
                HttpResponse::NotFound().body("未找到 company 字段") 
            }
        }
        Err(err) => {
            eprintln!("请求失败: {}", err);
            HttpResponse::InternalServerError().body(format!("请求失败: {}", err)) 
        }
    }
}

async fn top_holder_handler(path: web::Path<(String,)>) -> HttpResponse {
    let symbol = &path.0;  // 解构 path 中的元组，直接获取 symbol 参数

    // 创建 reqwest 客户端
    let client = Client::new();

    // 获取股票数据
    match get_top_holders(symbol, &client).await {
        Ok(data) => {
            // 返回结果作为 JSON 响应
            if let Some(items) = data.get("data").and_then(|data| data.get("items")) {
                HttpResponse::Ok().json(items)
            } else {
                HttpResponse::NotFound().body("未找到 items 字段")
            }
        }
        Err(err) => {
            eprintln!("请求失败: {}", err);
            HttpResponse::InternalServerError().body(format!("请求失败: {}", err))
        }
    }
}

async fn holders_num_handler(path: web::Path<(String,)>) -> HttpResponse {
    let symbol = &path.0;  // 解构 path 中的元组，直接获取 symbol 参数

    // 创建 reqwest 客户端
    let client = Client::new();

    // 获取股票数据
    match get_holders_nums(symbol, &client).await {
        Ok(data) => {
            // 返回结果作为 JSON 响应
            if let Some(items) = data.get("data").and_then(|data| data.get("items")) {
                HttpResponse::Ok().json(items)
            } else {
                HttpResponse::NotFound().body("未找到 items 字段")
            }
        }
        Err(err) => {
            eprintln!("请求失败: {}", err);
            HttpResponse::InternalServerError().body(format!("请求失败: {}", err))
        }
    }
}

async fn bonus_handler(path: web::Path<(String,)>) -> HttpResponse {
    let symbol = &path.0;  // 解构 path 中的元组，直接获取 symbol 参数

    // 创建 reqwest 客户端
    let client = Client::new();

    // 获取股票数据
    match get_bonus(symbol, &client).await {
        Ok(data) => {
            // 返回结果作为 JSON 响应
            if let Some(items) = data.get("data").and_then(|data| data.get("items")) {
                HttpResponse::Ok().json(items)
            } else {
                HttpResponse::NotFound().body("未找到 items 字段")
            }
        }
        Err(err) => {
            eprintln!("请求失败: {}", err);
            HttpResponse::InternalServerError().body(format!("请求失败: {}", err))
        }
    }
}

async fn indicator_handler(path: web::Path<(String,)>) -> HttpResponse {
    let symbol = &path.0;  // 解构 path 中的元组，直接获取 symbol 参数

    // 创建 reqwest 客户端
    let client = Client::new();

    // 获取股票数据
    match get_indicator(symbol, &client).await {
        Ok(data) => {
            // 返回结果作为 JSON 响应
            if let Some(list) = data.get("data").and_then(|data| data.get("list")) {
                HttpResponse::Ok().json(list)
            } else {
                HttpResponse::NotFound().body("未找到 list 字段")
            }
        }
        Err(err) => {
            eprintln!("请求失败: {}", err);
            HttpResponse::InternalServerError().body(format!("请求失败: {}", err))
        }
    }
}

async fn income_handler(path: web::Path<(String,)>) -> HttpResponse {
    let symbol = &path.0;  // 解构 path 中的元组，直接获取 symbol 参数

    // 创建 reqwest 客户端
    let client = Client::new();

    // 获取股票数据
    match get_income(symbol, &client).await {
        Ok(data) => {
            // 返回结果作为 JSON 响应
            if let Some(list) = data.get("data").and_then(|data| data.get("list")) {
                HttpResponse::Ok().json(list)
            } else {
                HttpResponse::NotFound().body("未找到 list 字段")
            }
        }
        Err(err) => {
            eprintln!("请求失败: {}", err);
            HttpResponse::InternalServerError().body(format!("请求失败: {}", err))
        }
    }
}

async fn balance_handler(path: web::Path<(String,)>) -> HttpResponse {
    let symbol = &path.0;  // 解构 path 中的元组，直接获取 symbol 参数

    // 创建 reqwest 客户端
    let client = Client::new();

    // 获取股票数据
    match get_balance(symbol, &client).await {
        Ok(data) => {
            // 返回结果作为 JSON 响应
            if let Some(list) = data.get("data").and_then(|data| data.get("list")) {
                HttpResponse::Ok().json(list)
            } else {
                HttpResponse::NotFound().body("未找到 list 字段")
            }
        }
        Err(err) => {
            eprintln!("请求失败: {}", err);
            HttpResponse::InternalServerError().body(format!("请求失败: {}", err))
        }
    }
}

async fn cash_flow_handler(path: web::Path<(String,)>) -> HttpResponse {
    let symbol = &path.0;  // 解构 path 中的元组，直接获取 symbol 参数

    // 创建 reqwest 客户端
    let client = Client::new();

    // 获取股票数据
    match get_cash_flow(symbol, &client).await {
        Ok(data) => {
            // 返回结果作为 JSON 响应
            if let Some(list) = data.get("data").and_then(|data| data.get("list")) {
                HttpResponse::Ok().json(list)
            } else {
                HttpResponse::NotFound().body("未找到 list 字段")
            }
        }
        Err(err) => {
            eprintln!("请求失败: {}", err);
            HttpResponse::InternalServerError().body(format!("请求失败: {}", err))
        }
    }
}
