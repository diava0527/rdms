//! 预算/成本模型：支撑“经济决策方法”——成本核算、预算执行、投入产出分析。
//!
//! 归属：成员 A

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// 成本类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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

/// 新增成本条目请求体
#[derive(Debug, Clone, Deserialize)]
pub struct NewBudget {
    pub project_id: i64,
    pub cost_type: CostType,
    pub amount: f64,
    pub note: String,
    pub occurred_at: NaiveDateTime,
}
