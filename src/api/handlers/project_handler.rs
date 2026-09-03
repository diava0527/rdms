//! 项目接口处理器。
//!
//! 归属：成员 B

use axum::extract::{Path, State};
use axum::Json;
use sqlx::SqlitePool;

use crate::error::ApiResult;
use crate::models::{NewProject, Project, UpdateProject};
use crate::services::{budget_service, project_service};

/// POST /api/projects
pub async fn create_project(
    State(pool): State<SqlitePool>,
    Json(payload): Json<NewProject>,
) -> ApiResult<Json<Project>> {
    todo!("实现：调用 project_service::create_project")
}

/// GET /api/projects
pub async fn list_projects(State(pool): State<SqlitePool>) -> ApiResult<Json<Vec<Project>>> {
    todo!("实现：调用 project_service::list_projects")
}

/// GET /api/projects/:id
pub async fn get_project(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> ApiResult<Json<Project>> {
    todo!("实现：调用 project_service::get_project")
}

/// PUT /api/projects/:id
pub async fn update_project(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateProject>,
) -> ApiResult<Json<Project>> {
    todo!("实现：调用 project_service::update_project")
}

/// DELETE /api/projects/:id
pub async fn delete_project(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> ApiResult<Json<()>> {
    todo!("实现：调用 project_service::delete_project")
}

/// GET /api/projects/:id/cost-summary —— 成本核算（经济决策）
pub async fn cost_summary(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> ApiResult<Json<budget_service::CostSummary>> {
    todo!("实现：调用 budget_service::summarize_cost")
}
