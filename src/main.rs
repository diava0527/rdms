//! 程序入口：负责初始化日志、加载配置、建立数据库连接池、启动 HTTP 服务。
//!
//! 归属：成员 A

use anyhow::Result;
use rdms::config::Config;
use rdms::db;
use rdms::api;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. 初始化日志（tracing）
    tracing_subscriber::fmt::init();

    // 2. 加载配置（数据库路径、监听地址等）
    let config = Config::from_env()?;

    // 3. 建立数据库连接池，并执行建表迁移
    let pool = db::init_pool(&config.database_url).await?;
    db::run_migrations(&pool).await?;

    // 4. 构建路由（含 REST 接口与前端静态文件服务）
    let router = api::create_router(pool, &config);

    // 5. 启动服务
    let listener = tokio::net::TcpListener::bind(&config.bind_addr).await?;
    tracing::info!("R&D 管理系统已启动，监听 {}", config.bind_addr);
    axum::serve(listener, router).await?;

    Ok(())
}
