use actix_web::{web, App, HttpServer, Responder};
use serde::Deserialize;
use handlers::handlers::generic_handler;
use services::stock_service::*;
use fern::Dispatch;
use log::LevelFilter;
use std::fs::OpenOptions;
use chrono::Local;

mod core;
mod services;
mod errors;
mod handlers;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // env_logger::Builder::from_default_env()
    //     .filter_level(log::LevelFilter::Info)
    //     .init();
    setup_logger().expect("Failed to initialize logger");

    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api")
                    .route("/stock/{symbol}", web::get().to(handle_stock))
                    .route("/company/{symbol}", web::get().to(handle_company))
                    .route("/top_holders/{symbol}", web::get().to(handle_top_holders))
                    .route("/holders/{symbol}", web::get().to(handle_holders))
                    .route("/bonus/{symbol}", web::get().to(handle_bonus))
                    .route("/indicator/{symbol}", web::get().to(handle_indicator))
                    .route("/income/{symbol}", web::get().to(handle_income))
                    .route("/balance/{symbol}", web::get().to(handle_balance))
                    .route("/cash_flow/{symbol}", web::get().to(handle_cash_flow))
                    .route("/minute/{symbol}", web::get().to(handle_minute_line))
                    .route("/trade/{symbol}", web::get().to(handle_trade))
                    .route("/pankou/{symbol}", web::get().to(handle_pankou))
                    .route("/kline/{symbol}/{period}", web::get().to(handle_k_line))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

fn setup_logger() -> Result<(), fern::InitError> {
    // 创建日志文件（自动追加）
    let log_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("budd.log")?;

    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(LevelFilter::Info)  // 全局日志级别
        .chain(std::io::stdout())  // 输出到控制台
        .chain(log_file)           // 输出到文件
        .apply()?;

    Ok(())
}


// 为每个路由创建明确的处理函数
async fn handle_stock(path: web::Path<String>) -> impl Responder {
    generic_handler(path, get_stock_data).await
}

async fn handle_company(path: web::Path<String>) -> impl Responder {
    generic_handler(path, get_company_data).await
}

async fn handle_top_holders(path: web::Path<String>) -> impl Responder {
    generic_handler(path, get_top_holders).await
}

async fn handle_holders(path: web::Path<String>) -> impl Responder {
    generic_handler(path, get_holders_nums).await
}

async fn handle_bonus(path: web::Path<String>) -> impl Responder {
    generic_handler(path, get_bonus).await
}

async fn handle_indicator(path: web::Path<String>) -> impl Responder {
    generic_handler(path, get_indicator).await
}

async fn handle_income(path: web::Path<String>) -> impl Responder {
    generic_handler(path, get_income).await
}

async fn handle_balance(path: web::Path<String>) -> impl Responder {
    generic_handler(path, get_balance).await
}

async fn handle_cash_flow(path: web::Path<String>) -> impl Responder {
    generic_handler(path, get_cash_flow).await
}

async fn handle_minute_line(path: web::Path<String>) -> impl Responder {
    generic_handler(path, get_minute_line).await
}

async fn handle_trade(path: web::Path<String>) -> impl Responder {
    generic_handler(path, get_trade).await
}

async fn handle_pankou(path: web::Path<String>) -> impl Responder {
    generic_handler(path, get_pankou).await
}

#[derive(Deserialize)]
pub struct KLineParams {
    symbol: String,
    period: String,
}

async fn handle_k_line(params: web::Path<KLineParams>) -> impl Responder {
    generic_handler(
        web::Path::from(params.symbol.clone()),
        move |sym, client| get_k_line(sym, client, params.period.clone())
    ).await
}