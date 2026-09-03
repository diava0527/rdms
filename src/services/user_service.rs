//! 成员管理业务逻辑。
//!
//! 归属：成员 A

use sqlx::SqlitePool;

use crate::error::ApiError;
use crate::models::{NewUser, UpdateUser, User};

/// 创建成员
pub async fn create_user(pool: &SqlitePool, new_user: NewUser) -> Result<User, ApiError> {
    todo!("实现：INSERT INTO users，返回新建成员")
}

/// 查询全部成员
pub async fn list_users(pool: &SqlitePool) -> Result<Vec<User>, ApiError> {
    todo!("实现：SELECT * FROM users ORDER BY id")
}

/// 查询单个成员
pub async fn get_user(pool: &SqlitePool, id: i64) -> Result<User, ApiError> {
    todo!("实现：SELECT * FROM users WHERE id = ?，未找到返回 NotFound")
}

/// 更新成员
pub async fn update_user(
    pool: &SqlitePool,
    id: i64,
    update: UpdateUser,
) -> Result<User, ApiError> {
    todo!("实现：动态更新非空字段，返回更新后的成员")
}

/// 删除成员
pub async fn delete_user(pool: &SqlitePool, id: i64) -> Result<(), ApiError> {
    todo!("实现：DELETE FROM users WHERE id = ?")
}
