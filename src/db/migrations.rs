//! 建表 SQL 语句集合。
//!
//! 归属：成员 A
//!
//! 说明：使用 `CREATE TABLE IF NOT EXISTS`，保证可重复执行。

/// 全部迁移语句（按依赖顺序执行）
pub const ALL_MIGRATIONS: &[&str] = &[
    // 成员表
    r#"
    CREATE TABLE IF NOT EXISTS users (
        id          INTEGER PRIMARY KEY AUTOINCREMENT,
        name        TEXT    NOT NULL,
        role        TEXT    NOT NULL,
        department  TEXT    NOT NULL,
        email       TEXT    NOT NULL UNIQUE,
        created_at  TEXT    NOT NULL DEFAULT (datetime('now'))
    );
    "#,
    // 项目表
    r#"
    CREATE TABLE IF NOT EXISTS projects (
        id          INTEGER PRIMARY KEY AUTOINCREMENT,
        name        TEXT    NOT NULL,
        description TEXT    NOT NULL DEFAULT '',
        status      TEXT    NOT NULL DEFAULT 'pending',
        manager_id  INTEGER NOT NULL REFERENCES users(id),
        start_date  TEXT,
        end_date    TEXT,
        milestones  TEXT    NOT NULL DEFAULT '[]',
        budget      REAL    NOT NULL DEFAULT 0,
        created_at  TEXT    NOT NULL DEFAULT (datetime('now'))
    );
    "#,
    // 任务表
    r#"
    CREATE TABLE IF NOT EXISTS tasks (
        id             INTEGER PRIMARY KEY AUTOINCREMENT,
        title          TEXT    NOT NULL,
        description    TEXT    NOT NULL DEFAULT '',
        project_id     INTEGER NOT NULL REFERENCES projects(id),
        assignee_id    INTEGER REFERENCES users(id),
        status         TEXT    NOT NULL DEFAULT 'todo',
        priority       TEXT    NOT NULL DEFAULT 'medium',
        estimate_hours REAL    NOT NULL DEFAULT 0,
        due_date       TEXT,
        created_at     TEXT    NOT NULL DEFAULT (datetime('now'))
    );
    "#,
    // 工时表
    r#"
    CREATE TABLE IF NOT EXISTS attendance (
        id         INTEGER PRIMARY KEY AUTOINCREMENT,
        user_id    INTEGER NOT NULL REFERENCES users(id),
        task_id    INTEGER REFERENCES tasks(id),
        work_date  TEXT    NOT NULL,
        hours      REAL    NOT NULL,
        note       TEXT    NOT NULL DEFAULT '',
        created_at TEXT    NOT NULL DEFAULT (datetime('now'))
    );
    "#,
    // 预算/成本表
    r#"
    CREATE TABLE IF NOT EXISTS budgets (
        id          INTEGER PRIMARY KEY AUTOINCREMENT,
        project_id  INTEGER NOT NULL REFERENCES projects(id),
        cost_type   TEXT    NOT NULL,
        amount      REAL    NOT NULL,
        note        TEXT    NOT NULL DEFAULT '',
        occurred_at TEXT    NOT NULL DEFAULT (datetime('now'))
    );
    "#,
];
