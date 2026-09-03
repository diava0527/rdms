//! 数据模型层：定义系统所有核心实体结构体。
//!
//! 归属：成员 A
//!
//! 说明：这些结构体同时是数据库表结构与 JSON 接口数据结构的“单一事实来源”，
//! 成员 B（接口层）与成员 C（前端）都以此为准对接。

pub mod user;
pub mod project;
pub mod task;
pub mod attendance;
pub mod budget;

pub use user::{User, Role, NewUser, UpdateUser};
pub use project::{Project, ProjectStatus, NewProject, UpdateProject};
pub use task::{Task, TaskStatus, TaskPriority, NewTask, UpdateTask};
pub use attendance::{Attendance, NewAttendance};
pub use budget::{Budget, CostType, NewBudget};
