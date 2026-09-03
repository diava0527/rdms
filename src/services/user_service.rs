//! 成员管理业务逻辑。
//!
//! 归属：成员 A

use sqlx::SqlitePool;

use crate::error::{ApiError, ApiResult};
use crate::models::user::{NewUser, RawUser, UpdateUser, User};

/// 创建成员
pub async fn create_user(pool: &SqlitePool, new_user: NewUser) -> ApiResult<User> {
    let result = sqlx::query(
        "INSERT INTO users (name, role, department, email) VALUES (?, ?, ?, ?)",
    )
    .bind(&new_user.name)
    .bind(new_user.role.as_str())
    .bind(&new_user.department)
    .bind(&new_user.email)
    .execute(pool)
    .await?;

    get_user(pool, result.last_insert_rowid()).await
}

/// 查询全部成员
pub async fn list_users(pool: &SqlitePool) -> ApiResult<Vec<User>> {
    let rows = sqlx::query_as::<_, RawUser>("SELECT * FROM users ORDER BY id")
        .fetch_all(pool)
        .await?;

    rows.into_iter().map(|r| User::try_from(r)).collect()
}

/// 查询单个成员
pub async fn get_user(pool: &SqlitePool, id: i64) -> ApiResult<User> {
    let row = sqlx::query_as::<_, RawUser>("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?;

    match row {
        Some(raw) => User::try_from(raw),
        None => Err(ApiError::NotFound(format!("成员 {id} 不存在"))),
    }
}

/// 更新成员（未提供的字段保持原值）
pub async fn update_user(pool: &SqlitePool, id: i64, update: UpdateUser) -> ApiResult<User> {
    let existing = get_user(pool, id).await?;

    let name = update.name.unwrap_or(existing.name);
    let role = update.role.unwrap_or(existing.role);
    let department = update.department.unwrap_or(existing.department);
    let email = update.email.unwrap_or(existing.email);

    sqlx::query("UPDATE users SET name = ?, role = ?, department = ?, email = ? WHERE id = ?")
        .bind(&name)
        .bind(role.as_str())
        .bind(&department)
        .bind(&email)
        .bind(id)
        .execute(pool)
        .await?;

    get_user(pool, id).await
}

/// 删除成员
pub async fn delete_user(pool: &SqlitePool, id: i64) -> ApiResult<()> {
    let result = sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound(format!("成员 {id} 不存在")));
    }
    Ok(())
}
