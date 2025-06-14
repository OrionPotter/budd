# 项目概述

这个项目是一个基于 Actix-web 的 HTTP 服务器，用于获取股票数据。它通过调用雪球 API 来获取指定股票的报价信息。

## 目录结构

- `src/main.rs`: 项目的入口文件，负责启动 HTTP 服务器并设置路由。
- `src/core/common_header.rs`: 定义了创建 HTTP 请求头的函数。
- `src/services/stock_service.rs`: 包含获取股票数据的服务逻辑。
- `src/errors/error.rs`: 定义了项目中使用的错误类型。

## 使用方法

1. 确保已安装 Rust 和 Cargo。
2. 克隆此仓库并进入项目目录。
3. 运行以下命令启动服务器：

   ```bash
   cargo run
   ```