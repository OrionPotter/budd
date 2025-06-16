use actix_web::{web, App, HttpServer, Responder};
use handlers::handlers::generic_handler;
use services::stock_service::*;

mod core;
mod services;
mod errors;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

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
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
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