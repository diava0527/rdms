//! 任务接口处理器。
//!
//! 归属：成员 B

use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::error::ApiResult;
use crate::models::{NewTask, Task, UpdateTask};
use crate::services::task_service;

/// GET /api/tasks 的查询参数
#[derive(Debug, Deserialize)]
pub struct TaskQuery {
    pub project_id: Option<i64>,
}

/// POST /api/tasks
pub async fn create_task(
    State(pool): State<SqlitePool>,
    Json(payload): Json<NewTask>,
) -> ApiResult<Json<Task>> {
    todo!("实现：调用 task_service::create_task")
}

/// GET /api/tasks?project_id=1
pub async fn list_tasks(
    State(pool): State<SqlitePool>,
    Query(q): Query<TaskQuery>,
) -> ApiResult<Json<Vec<Task>>> {
    todo!("实现：调用 task_service::list_tasks(pool, q.project_id)")
}

/// GET /api/tasks/:id
pub async fn get_task(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> ApiResult<Json<Task>> {
    todo!("实现：调用 task_service::get_task")
}

/// PUT /api/tasks/:id
pub async fn update_task(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateTask>,
) -> ApiResult<Json<Task>> {
    todo!("实现：调用 task_service::update_task")
}

/// DELETE /api/tasks/:id
pub async fn delete_task(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> ApiResult<Json<()>> {
    todo!("实现：调用 task_service::delete_task")
}
