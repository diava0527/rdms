//! 预算/成本接口处理器。
//!
//! 归属：成员 B

use axum::extract::{Query, State};
use axum::Json;
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::error::{ApiError, ApiResult};
use crate::models::{Budget, NewBudget};
use crate::services::budget_service;

/// GET /api/budgets 的查询参数
#[derive(Debug, Deserialize)]
pub struct BudgetQuery {
    pub project_id: i64,
}

/// POST /api/budgets
pub async fn create_budget(
    State(pool): State<SqlitePool>,
    payload: Result<Json<NewBudget>, axum::extract::rejection::JsonRejection>,
) -> ApiResult<Json<Budget>> {
    let Json(payload) = payload.map_err(ApiError::from)?;
    Ok(Json(budget_service::create_budget(&pool, payload).await?))
}

/// GET /api/budgets?project_id=1
pub async fn list_budget(
    State(pool): State<SqlitePool>,
    q: Result<Query<BudgetQuery>, axum::extract::rejection::QueryRejection>,
) -> ApiResult<Json<Vec<Budget>>> {
    let Query(q) = q.map_err(ApiError::from)?;
    Ok(Json(
        budget_service::list_budget(&pool, q.project_id).await?,
    ))
}
