//! 项目接口处理器。
//!
//! 归属：成员 B

use axum::extract::{Path, State};
use axum::Json;
use sqlx::SqlitePool;

use crate::error::{ApiError, ApiResult};
use crate::models::{NewProject, Project, UpdateProject};
use crate::services::{budget_service, project_service};

/// POST /api/projects
pub async fn create_project(
    State(pool): State<SqlitePool>,
    payload: Result<Json<NewProject>, axum::extract::rejection::JsonRejection>,
) -> ApiResult<Json<Project>> {
    let Json(payload) = payload.map_err(ApiError::from)?;
    Ok(Json(project_service::create_project(&pool, payload).await?))
}

/// GET /api/projects
pub async fn list_projects(State(pool): State<SqlitePool>) -> ApiResult<Json<Vec<Project>>> {
    Ok(Json(project_service::list_projects(&pool).await?))
}

/// GET /api/projects/:id
pub async fn get_project(
    State(pool): State<SqlitePool>,
    id: Result<Path<i64>, axum::extract::rejection::PathRejection>,
) -> ApiResult<Json<Project>> {
    let Path(id) = id.map_err(ApiError::from)?;
    Ok(Json(project_service::get_project(&pool, id).await?))
}

/// PUT /api/projects/:id
pub async fn update_project(
    State(pool): State<SqlitePool>,
    id: Result<Path<i64>, axum::extract::rejection::PathRejection>,
    payload: Result<Json<UpdateProject>, axum::extract::rejection::JsonRejection>,
) -> ApiResult<Json<Project>> {
    let Path(id) = id.map_err(ApiError::from)?;
    let Json(payload) = payload.map_err(ApiError::from)?;
    Ok(Json(
        project_service::update_project(&pool, id, payload).await?,
    ))
}

/// DELETE /api/projects/:id
pub async fn delete_project(
    State(pool): State<SqlitePool>,
    id: Result<Path<i64>, axum::extract::rejection::PathRejection>,
) -> ApiResult<axum::http::StatusCode> {
    let Path(id) = id.map_err(ApiError::from)?;
    project_service::delete_project(&pool, id).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

/// GET /api/projects/:id/cost-summary —— 成本核算（经济决策）
pub async fn cost_summary(
    State(pool): State<SqlitePool>,
    id: Result<Path<i64>, axum::extract::rejection::PathRejection>,
) -> ApiResult<Json<budget_service::CostSummary>> {
    let Path(id) = id.map_err(ApiError::from)?;
    Ok(Json(budget_service::summarize_cost(&pool, id).await?))
}
