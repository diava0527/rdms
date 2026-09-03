//! 预算/成本与“经济决策”业务逻辑。
//!
//! 归属：成员 A
//!
//! 该模块对应课程要求③中“经济决策方法”的核心实现：
//! - 成本核算：统计项目实际成本
//! - 预算执行率：实际成本 / 项目预算
//! - 投资回报（ROI）：(收益 - 成本) / 成本

use sqlx::SqlitePool;

use crate::error::ApiError;
use crate::models::{Budget, NewBudget};

/// 记录一条成本
pub async fn create_budget(pool: &SqlitePool, new_budget: NewBudget) -> Result<Budget, ApiError> {
    todo!("实现：INSERT INTO budgets")
}

/// 查询某项目的成本明细
pub async fn list_budget(pool: &SqlitePool, project_id: i64) -> Result<Vec<Budget>, ApiError> {
    todo!("实现：SELECT * FROM budgets WHERE project_id = ?")
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
pub async fn summarize_cost(pool: &SqlitePool, project_id: i64) -> Result<CostSummary, ApiError> {
    todo!("实现：JOIN budgets 统计实际成本，读取 projects.budget 计算执行率")
}

/// 投资回报率（ROI）计算：需要外部给定“项目收益”。
pub fn calc_roi(project_income: f64, actual_cost: f64) -> f64 {
    todo!("实现：ROI = (project_income - actual_cost) / actual_cost")
}
