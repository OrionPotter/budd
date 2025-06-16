
# 股票数据API服务

基于Actix-web的Rust异步HTTP服务，提供雪球财经数据的统一访问接口。

## 功能特性

- ✅ 统一RESTful API响应格式
- ⚡ 全异步高性能处理
- 📊 支持多种财务数据查询
- 🔒 内置请求日志和错误处理
- 📦 模块化代码结构

## 数据接口

| 端点 | 描述 | 示例 |
|------|------|------|
| `GET /api/stock/{symbol}` | 股票实时报价 | `/api/stock/SH600519` |
| `GET /api/company/{symbol}` | 公司基本信息 | `/api/company/SZ000001` |
| `GET /api/top_holders/{symbol}` | 十大股东 | `/api/top_holders/USAAPL` |
| `GET /api/indicator/{symbol}` | 财务指标 | `/api/indicator/HK00700` |
| `GET /api/income/{symbol}` | 利润表数据 | `/api/income/SH601318` |

## 响应格式

成功响应：
```json
{
  "success": true,
  "message": "success",
  "data": []
}
```

错误响应：
```json
{
  "success": false,
  "message": "错误描述",
  "data": null
}
```

## 技术栈

- **框架**: Actix-web 4.x
- **HTTP客户端**: Reqwest
- **JSON处理**: Serde
- **日志**: env_logger
- **异步**: Tokio runtime

## 项目结构

```
src/
├── main.rs                # 服务入口和路由
├── core/                  # 核心组件
│   ├── common_header.rs   # HTTP头处理
│   └── response.rs        # 统一响应格式
├── services/              # 业务服务
│   └── stock_service.rs   # 雪球API调用
├── errors/                # 错误处理
│   └── error.rs           # 自定义错误类型
└── handlers/              # 请求处理器
    └── handlers.rs        # 通用请求处理
```

## 快速开始

### 运行服务

1. 设置环境变量（可选）：
   ```bash
   export XQ_A_TOKEN=your_token
   export XQ_R_TOKEN=your_refresh_token
   ```

2. 启动服务：
   ```bash
   cargo run
   ```

3. 测试接口：
   ```bash
   curl http://localhost:8080/api/stock/SH600519
   ```

### 开发环境

- Rust 1.65+
- Cargo
- 雪球API访问权限

## 部署建议

1. 生产环境构建：
   ```bash
   cargo build --release
   ```

2. 使用systemd管理服务：
   ```ini
   [Unit]
   Description=Stock API Service
   
   [Service]
   ExecStart=/path/to/binary
   Environment=XQ_A_TOKEN=prod_token
   Environment=XQ_R_TOKEN=prod_refresh_token
   Restart=always
   
   [Install]
   WantedBy=multi-user.target
   ```

## 性能优化

- 添加Redis缓存层
- 实现请求限流
- 启用HTTP压缩
- 使用连接池管理数据库连接

## 贡献指南

欢迎提交Issue和PR！请确保：
1. 代码通过cargo clippy检查
2. 添加适当的单元测试
3. 更新相关文档