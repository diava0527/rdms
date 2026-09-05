//! 任务管理业务逻辑。
//!
//! 归属：成员 A

use sqlx::SqlitePool;

use crate::error::{ApiError, ApiResult};
use crate::models::task::{NewTask, RawTask, Task, TaskStatus, UpdateTask};

/// 判断状态流转是否合法（只允许向前推进）。
fn can_transition(from: TaskStatus, to: TaskStatus) -> bool {
    use TaskStatus::*;
    matches!(
        (from, to),
        (Todo, InProgress)
            | (Todo, InReview)
            | (Todo, Done)
            | (InProgress, InReview)
            | (InProgress, Done)
            | (InReview, Done)
            | (Done, Done)
    )
}

/// 创建任务（初始状态为 Todo）
pub async fn create_task(pool: &SqlitePool, new_task: NewTask) -> ApiResult<Task> {
    let result = sqlx::query(
        "INSERT INTO tasks (title, description, project_id, assignee_id, priority, estimate_hours, due_date) \
         VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&new_task.title)
    .bind(&new_task.description)
    .bind(new_task.project_id)
    .bind(new_task.assignee_id)
    .bind(new_task.priority.as_str())
    .bind(new_task.estimate_hours)
    .bind(new_task.due_date)
    .execute(pool)
    .await?;

    get_task(pool, result.last_insert_rowid()).await
}

/// 查询任务（可按 project_id 过滤，None 表示全部）
pub async fn list_tasks(pool: &SqlitePool, project_id: Option<i64>) -> ApiResult<Vec<Task>> {
    let rows = sqlx::query_as::<_, RawTask>(
        "SELECT * FROM tasks WHERE (? IS NULL OR project_id = ?) ORDER BY id",
    )
    .bind(project_id)
    .bind(project_id)
    .fetch_all(pool)
    .await?;

    rows.into_iter().map(|r| Task::try_from(r)).collect()
}

/// 查询单个任务
pub async fn get_task(pool: &SqlitePool, id: i64) -> ApiResult<Task> {
    let row = sqlx::query_as::<_, RawTask>("SELECT * FROM tasks WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?;

    match row {
        Some(raw) => Task::try_from(raw),
        None => Err(ApiError::NotFound(format!("任务 {id} 不存在"))),
    }
}

/// 更新任务（含状态流转校验）
pub async fn update_task(pool: &SqlitePool, id: i64, update: UpdateTask) -> ApiResult<Task> {
    let existing = get_task(pool, id).await?;

    let title = update.title.unwrap_or(existing.title);
    let description = update.description.unwrap_or(existing.description);
    let assignee_id = update.assignee_id.or(existing.assignee_id);
    let priority = update.priority.unwrap_or(existing.priority);
    let estimate_hours = update.estimate_hours.unwrap_or(existing.estimate_hours);
    let due_date = update.due_date.or(existing.due_date);

    // 状态流转校验
    let status = match update.status {
        Some(new_status) if new_status != existing.status => {
            if !can_transition(existing.status, new_status) {
                return Err(ApiError::BadRequest(format!(
                    "非法状态流转: {:?} -> {:?}",
                    existing.status, new_status
                )));
            }
            new_status
        }
        _ => existing.status,
    };

    sqlx::query(
        "UPDATE tasks SET title = ?, description = ?, assignee_id = ?, status = ?, \
         priority = ?, estimate_hours = ?, due_date = ? WHERE id = ?",
    )
    .bind(&title)
    .bind(&description)
    .bind(assignee_id)
    .bind(status.as_str())
    .bind(priority.as_str())
    .bind(estimate_hours)
    .bind(due_date)
    .bind(id)
    .execute(pool)
    .await?;

    get_task(pool, id).await
}

/// 删除任务
pub async fn delete_task(pool: &SqlitePool, id: i64) -> ApiResult<()> {
    let result = sqlx::query("DELETE FROM tasks WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound(format!("任务 {id} 不存在")));
    }
    Ok(())
}
