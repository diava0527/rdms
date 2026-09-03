//! 项目管理业务逻辑。
//!
//! 归属：成员 A

use sqlx::SqlitePool;

use crate::error::{ApiError, ApiResult};
use crate::models::project::{NewProject, RawProject, UpdateProject, Project};

/// 创建项目
pub async fn create_project(pool: &SqlitePool, new_project: NewProject) -> ApiResult<Project> {
    let milestones = serde_json::to_string(&new_project.milestones)?;

    let result = sqlx::query(
        "INSERT INTO projects (name, description, manager_id, start_date, end_date, milestones, budget) \
         VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&new_project.name)
    .bind(&new_project.description)
    .bind(new_project.manager_id)
    .bind(new_project.start_date)
    .bind(new_project.end_date)
    .bind(&milestones)
    .bind(new_project.budget)
    .execute(pool)
    .await?;

    get_project(pool, result.last_insert_rowid()).await
}

/// 查询全部项目
pub async fn list_projects(pool: &SqlitePool) -> ApiResult<Vec<Project>> {
    let rows = sqlx::query_as::<_, RawProject>("SELECT * FROM projects ORDER BY id")
        .fetch_all(pool)
        .await?;

    rows.into_iter().map(|r| Project::try_from(r)).collect()
}

/// 查询单个项目
pub async fn get_project(pool: &SqlitePool, id: i64) -> ApiResult<Project> {
    let row = sqlx::query_as::<_, RawProject>("SELECT * FROM projects WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?;

    match row {
        Some(raw) => Project::try_from(raw),
        None => Err(ApiError::NotFound(format!("项目 {id} 不存在"))),
    }
}

/// 更新项目（未提供的字段保持原值）
pub async fn update_project(
    pool: &SqlitePool,
    id: i64,
    update: UpdateProject,
) -> ApiResult<Project> {
    let existing = get_project(pool, id).await?;

    let name = update.name.unwrap_or(existing.name);
    let description = update.description.unwrap_or(existing.description);
    let status = update.status.unwrap_or(existing.status);
    let manager_id = update.manager_id.unwrap_or(existing.manager_id);
    let start_date = update.start_date.or(existing.start_date);
    let end_date = update.end_date.or(existing.end_date);
    let milestones = update.milestones.unwrap_or(existing.milestones);
    let budget = update.budget.unwrap_or(existing.budget);

    let milestones_json = serde_json::to_string(&milestones)?;

    sqlx::query(
        "UPDATE projects SET name = ?, description = ?, status = ?, manager_id = ?, \
         start_date = ?, end_date = ?, milestones = ?, budget = ? WHERE id = ?",
    )
    .bind(&name)
    .bind(&description)
    .bind(status.as_str())
    .bind(manager_id)
    .bind(start_date)
    .bind(end_date)
    .bind(&milestones_json)
    .bind(budget)
    .bind(id)
    .execute(pool)
    .await?;

    get_project(pool, id).await
}

/// 删除项目
pub async fn delete_project(pool: &SqlitePool, id: i64) -> ApiResult<()> {
    let result = sqlx::query("DELETE FROM projects WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound(format!("项目 {id} 不存在")));
    }
    Ok(())
}
