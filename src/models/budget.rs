//! 预算/成本模型：支撑“经济决策方法”——成本核算、预算执行、投入产出分析。
//!
//! 归属：成员 A

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::error::ApiError;

/// 成本类型
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CostType {
    /// 人力成本
    Labor,
    /// 设备/物料成本
    Equipment,
    /// 外包/服务成本
    Outsourcing,
    /// 其他
    Other,
}

impl CostType {
    pub fn as_str(&self) -> &'static str {
        match self {
            CostType::Labor => "labor",
            CostType::Equipment => "equipment",
            CostType::Outsourcing => "outsourcing",
            CostType::Other => "other",
        }
    }
}

impl std::str::FromStr for CostType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "labor" => Ok(CostType::Labor),
            "equipment" => Ok(CostType::Equipment),
            "outsourcing" => Ok(CostType::Outsourcing),
            "other" => Ok(CostType::Other),
            other => Err(format!("未知成本类型: {other}")),
        }
    }
}

/// 预算/成本条目实体（对应数据库 budgets 表）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Budget {
    pub id: i64,
    /// 关联项目 ID
    pub project_id: i64,
    /// 成本类型
    pub cost_type: CostType,
    /// 金额（元）
    pub amount: f64,
    /// 说明
    pub note: String,
    /// 发生日期
    pub occurred_at: NaiveDateTime,
}

/// 数据库行 → 实体 的中间结构（枚举列以字符串读取）
#[derive(sqlx::FromRow)]
pub(crate) struct RawBudget {
    id: i64,
    project_id: i64,
    cost_type: String,
    amount: f64,
    note: String,
    occurred_at: NaiveDateTime,
}

impl TryFrom<RawBudget> for Budget {
    type Error = ApiError;
    fn try_from(r: RawBudget) -> Result<Self, Self::Error> {
        Ok(Budget {
            id: r.id,
            project_id: r.project_id,
            cost_type: r.cost_type.parse().map_err(ApiError::Internal)?,
            amount: r.amount,
            note: r.note,
            occurred_at: r.occurred_at,
        })
    }
}

/// 新增成本条目请求体
#[derive(Debug, Clone, Deserialize)]
pub struct NewBudget {
    pub project_id: i64,
    pub cost_type: CostType,
    pub amount: f64,
    pub note: String,
    pub occurred_at: NaiveDateTime,
}
