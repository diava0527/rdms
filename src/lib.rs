//! 库入口：集中声明各子模块，供 main.rs 与集成测试使用。
//!
//! 归属：成员 A

pub mod config;
pub mod error;
pub mod models;
pub mod db;
pub mod services;
pub mod api;
