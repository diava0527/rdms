//! 数据库层：连接池初始化与建表迁移。
//!
//! 归属：成员 A

pub mod migrations;

use anyhow::Result;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

/// 建立 SQLite 连接池。
pub async fn init_pool(database_url: &str) -> Result<SqlitePool> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    Ok(pool)
}

/// 执行建表迁移（幂等：表已存在则跳过）。
pub async fn run_migrations(pool: &SqlitePool) -> Result<()> {
    for stmt in migrations::ALL_MIGRATIONS {
        sqlx::query(stmt).execute(pool).await?;
    }
    Ok(())
}
