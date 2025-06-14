use actix_web::{web, App, HttpServer, HttpResponse};
use reqwest::Client;
use crate::services::stock_service::{get_company_data, get_stock_data};

mod core;
mod services;
mod errors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 启动 HTTP 服务器并绑定到指定地址
    HttpServer::new(|| {
        App::new()
            .route("/stock/{symbol}", web::get().to(stock_handler)) 
            .route("/company/{symbol}", web::get().to(company_handler))
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
