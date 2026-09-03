//! 应用配置：从环境变量读取配置项。
//!
//! 归属：成员 A

use anyhow::Result;

/// 全局配置
#[derive(Debug, Clone)]
pub struct Config {
    /// 监听地址，如 "127.0.0.1:8080"
    pub bind_addr: String,
    /// SQLite 数据库连接串，如 "sqlite://rdms.db"
    pub database_url: String,
    /// 前端静态资源目录（相对项目根目录）
    pub static_dir: String,
}

impl Config {
    /// 从环境变量加载配置（未设置时使用默认值）。
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            bind_addr: std::env::var("RDMS_BIND_ADDR")
                .unwrap_or_else(|_| "127.0.0.1:8080".to_string()),
            database_url: std::env::var("RDMS_DATABASE_URL")
                .unwrap_or_else(|_| "sqlite://rdms.db".to_string()),
            static_dir: std::env::var("RDMS_STATIC_DIR")
                .unwrap_or_else(|_| "frontend".to_string()),
        })
    }
}
