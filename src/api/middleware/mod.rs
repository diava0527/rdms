//! 中间件：CORS、日志与鉴权扩展点。
//!
//! 归属：成员 B

mod auth;

pub use auth::{cors_layer, trace_layer};
