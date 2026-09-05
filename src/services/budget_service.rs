//! 预算/成本与“经济决策”业务逻辑。
//!
//! 归属：成员 A
//!
//! 该模块对应课程要求③中“经济决策方法”的核心实现：
//! - 成本核算：统计项目实际成本
//! - 预算执行率：实际成本 / 项目预算
//! - 投资回报（ROI）：(收益 - 成本) / 成本

use sqlx::SqlitePool;

use crate::error::{ApiError, ApiResult};
use crate::models::budget::{Budget, NewBudget, RawBudget};

/// 记录一条成本
pub async fn create_budget(pool: &SqlitePool, new_budget: NewBudget) -> ApiResult<Budget> {
    let result = sqlx::query(
        "INSERT INTO budgets (project_id, cost_type, amount, note, occurred_at) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(new_budget.project_id)
    .bind(new_budget.cost_type.as_str())
    .bind(new_budget.amount)
    .bind(&new_budget.note)
    .bind(new_budget.occurred_at)
    .execute(pool)
    .await?;

    let row = sqlx::query_as::<_, RawBudget>("SELECT * FROM budgets WHERE id = ?")
        .bind(result.last_insert_rowid())
        .fetch_one(pool)
        .await?;

    Budget::try_from(row)
}

/// 查询某项目的成本明细
pub async fn list_budget(pool: &SqlitePool, project_id: i64) -> ApiResult<Vec<Budget>> {
    let rows = sqlx::query_as::<_, RawBudget>(
        "SELECT * FROM budgets WHERE project_id = ? ORDER BY occurred_at DESC",
    )
    .bind(project_id)
    .fetch_all(pool)
    .await?;

    rows.into_iter().map(|r| Budget::try_from(r)).collect()
}

/// 成本汇总结果
#[derive(Debug, Clone, serde::Serialize)]
pub struct CostSummary {
    pub project_id: i64,
    /// 项目预算（来自 projects.budget）
    pub total_budget: f64,
    /// 实际成本合计
    pub actual_cost: f64,
    /// 预算执行率 = actual_cost / total_budget
    pub budget_usage_rate: f64,
}

/// 成本核算：统计某项目实际成本并与预算对比。
pub async fn summarize_cost(pool: &SqlitePool, project_id: i64) -> ApiResult<CostSummary> {
    // 1. 读取项目预算
    let budget_row: Option<(f64,)> =
        sqlx::query_as("SELECT budget FROM projects WHERE id = ?")
            .bind(project_id)
            .fetch_optional(pool)
            .await?;

    let total_budget = match budget_row {
        Some((b,)) => b,
        None => return Err(ApiError::NotFound(format!("项目 {project_id} 不存在"))),
    };

    // 2. 汇总实际成本
    let cost_row: (Option<f64>,) =
        sqlx::query_as("SELECT SUM(amount) FROM budgets WHERE project_id = ?")
            .bind(project_id)
            .fetch_one(pool)
            .await?;

    let actual_cost = cost_row.0.unwrap_or(0.0);

    // 3. 计算预算执行率
    let budget_usage_rate = if total_budget == 0.0 {
        0.0
    } else {
        actual_cost / total_budget
    };

    Ok(CostSummary {
        project_id,
        total_budget,
        actual_cost,
        budget_usage_rate,
    })
}

/// 投资回报率（ROI）计算：需要外部给定“项目收益”。
pub fn calc_roi(project_income: f64, actual_cost: f64) -> f64 {
    if actual_cost == 0.0 {
        0.0
    } else {
        (project_income - actual_cost) / actual_cost
    }
}
