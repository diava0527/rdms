//! HTTP 接口层：路由注册、处理器、中间件。
//!
//! 归属：成员 B

pub mod router;
pub mod handlers;
pub mod middleware;

pub use router::create_router;
