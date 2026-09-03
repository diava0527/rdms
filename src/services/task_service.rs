//! 任务管理业务逻辑。
//!
//! 归属：成员 A

use sqlx::SqlitePool;

use crate::error::ApiError;
use crate::models::{NewTask, Task, UpdateTask};

/// 创建任务
pub async fn create_task(pool: &SqlitePool, new_task: NewTask) -> Result<Task, ApiError> {
    todo!("实现：INSERT INTO tasks")
}

/// 查询全部任务（可按 project_id 过滤）
pub async fn list_tasks(pool: &SqlitePool, project_id: Option<i64>) -> Result<Vec<Task>, ApiError> {
    todo!("实现：SELECT * FROM tasks WHERE (? IS NULL OR project_id = ?)")
}

/// 查询单个任务
pub async fn get_task(pool: &SqlitePool, id: i64) -> Result<Task, ApiError> {
    todo!("实现：按 id 查询")
}

/// 更新任务（含状态流转：Todo -> InProgress -> InReview -> Done）
pub async fn update_task(
    pool: &SqlitePool,
    id: i64,
    update: UpdateTask,
) -> Result<Task, ApiError> {
    todo!("实现：动态更新非空字段，并校验状态流转是否合法")
}

/// 删除任务
pub async fn delete_task(pool: &SqlitePool, id: i64) -> Result<(), ApiError> {
    todo!("实现：DELETE FROM tasks WHERE id = ?")
}
