//! 预算/成本接口处理器。
//!
//! 归属：成员 B

use axum::extract::{Query, State};
use axum::Json;
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::error::ApiResult;
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
    Json(payload): Json<NewBudget>,
) -> ApiResult<Json<Budget>> {
    todo!("实现：调用 budget_service::create_budget")
}

/// GET /api/budgets?project_id=1
pub async fn list_budget(
    State(pool): State<SqlitePool>,
    Query(q): Query<BudgetQuery>,
) -> ApiResult<Json<Vec<Budget>>> {
    todo!("实现：调用 budget_service::list_budget(pool, q.project_id)")
}
