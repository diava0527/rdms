//! 项目管理业务逻辑。
//!
//! 归属：成员 A

use sqlx::SqlitePool;

use crate::error::ApiError;
use crate::models::{NewProject, Project, UpdateProject};

/// 创建项目
pub async fn create_project(pool: &SqlitePool, new_project: NewProject) -> Result<Project, ApiError> {
    todo!("实现：将 milestones 序列化为 JSON 后 INSERT")
}

/// 查询全部项目
pub async fn list_projects(pool: &SqlitePool) -> Result<Vec<Project>, ApiError> {
    todo!("实现：SELECT * FROM projects ORDER BY id")
}

/// 查询单个项目
pub async fn get_project(pool: &SqlitePool, id: i64) -> Result<Project, ApiError> {
    todo!("实现：按 id 查询")
}

/// 更新项目
pub async fn update_project(
    pool: &SqlitePool,
    id: i64,
    update: UpdateProject,
) -> Result<Project, ApiError> {
    todo!("实现：动态更新非空字段")
}

/// 删除项目
pub async fn delete_project(pool: &SqlitePool, id: i64) -> Result<(), ApiError> {
    todo!("实现：DELETE FROM projects WHERE id = ?")
}
