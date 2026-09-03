//! 路由汇总：REST 接口 + 前端静态文件服务。
//!
//! 归属：成员 B
//!
//! 接口约定（成员 C 前端依据此对接）：
//!   GET    /api/users          成员列表
//!   POST   /api/users          新增成员
//!   GET    /api/users/:id      成员详情
//!   PUT    /api/users/:id      更新成员
//!   DELETE /api/users/:id      删除成员
//!   （projects / tasks / attendance / budgets 同理）
//!   GET    /api/projects/:id/cost-summary  成本核算（经济决策）
//!   GET    /                    返回前端 index.html
//!   /static/*                  前端静态资源

use axum::routing::{get, post, put, delete};
use axum::Router;
use sqlx::SqlitePool;
use tower_http::services::ServeDir;

use crate::config::Config;
use crate::api::handlers;
use crate::api::middleware;

/// 构建应用路由
pub fn create_router(pool: SqlitePool, config: &Config) -> Router {
    // REST 接口
    let api = Router::new()
        .route("/users", get(handlers::list_users).post(handlers::create_user))
        .route(
            "/users/:id",
            get(handlers::get_user)
                .put(handlers::update_user)
                .delete(handlers::delete_user),
        )
        .route(
            "/projects",
            get(handlers::list_projects).post(handlers::create_project),
        )
        .route(
            "/projects/:id",
            get(handlers::get_project)
                .put(handlers::update_project)
                .delete(handlers::delete_project),
        )
        .route(
            "/projects/:id/cost-summary",
            get(handlers::cost_summary),
        )
        .route("/tasks", get(handlers::list_tasks).post(handlers::create_task))
        .route(
            "/tasks/:id",
            get(handlers::get_task)
                .put(handlers::update_task)
                .delete(handlers::delete_task),
        )
        .route(
            "/attendance",
            get(handlers::list_attendance).post(handlers::create_attendance),
        )
        .route(
            "/budgets",
            get(handlers::list_budget).post(handlers::create_budget),
        );

    Router::new()
        .nest("/api", api)
        // 静态文件服务：托管 frontend/ 目录
        .nest_service("/", ServeDir::new(&config.static_dir))
        // 应用状态 + 中间件（鉴权、日志）
        .with_state(pool)
        .layer(middleware::cors_layer())
        .layer(middleware::trace_layer())
}
