//! 业务逻辑层：封装对数据库的增删改查与业务规则。
//!
//! 归属：成员 A
//!
//! 约定：所有函数第一个参数为 `&SqlitePool`，返回 `Result<T>`（`rdms::error::ApiError`）。
//! 成员 B 在 handler 中直接调用这些函数，无需关心 SQL 细节。

pub mod user_service;
pub mod project_service;
pub mod task_service;
pub mod attendance_service;
pub mod budget_service;
