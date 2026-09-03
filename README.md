# 企业研发团队内部管理系统（R&D Management System）

> 企业劳动实践课程作业 —— 【计科 + 大禹】3 人小组协作项目
> 本仓库为**架构框架（骨架）**，只定义结构、接口与数据模型，**不含具体实现**，由三位同学按分工完成。

---

## 一、项目简介

面向企业研发团队，提供**成员、项目、任务、工时、成本**的一体化管理工具，覆盖研发团队日常协作与经营决策场景。系统采用前后端分离架构：

- **后端（核心逻辑）**：Rust 实现，负责数据存储与业务规则；
- **前端（用户交互界面）**：原生 HTML / CSS / JavaScript，负责可视化展示与交互。

功能模块与课程要求的对应关系：

| 功能模块 | 说明 | 对应课程要求 |
| --- | --- | --- |
| 成员管理 | 研发人员增删改查、角色/部门划分 | ① 开发工具 |
| 项目管理 | 项目进度、里程碑、负责人、预算 | ③ 工程管理原理 |
| 任务管理 | 任务下发、认领、状态流转、优先级 | ③ 工程管理原理 |
| 工时管理 | 工时填报与统计 | ③ 工程管理原理 |
| 成本与决策 | 成本核算、预算执行率、ROI 投资回报 | ③ 经济决策方法 |

---

## 二、技术栈

| 层次 | 技术 | 说明 |
| --- | --- | --- |
| 后端语言 | Rust | 核心逻辑 |
| Web 框架 | axum | 路由、状态管理、JSON 接口 |
| 数据库 | SQLite + sqlx | 零配置、异步连接池 |
| 序列化 | serde / serde_json | 请求体与响应体 JSON 序列化 |
| 前端 | 原生 HTML/CSS/JS | 无需构建工具，由后端静态托管 |
| 开发工具 | cargo / git | 编译与版本协作 |

---

## 三、系统架构

```
┌──────────────────────────────────────────────────────────┐
│                      浏览器（用户交互界面）                │
│   frontend/  index.html · css/ · js/                     │
│   ── 成员 C 负责 ──                                      │
└──────────────────────────┬───────────────────────────────┘
                           │ HTTP / JSON (REST)
┌──────────────────────────▼───────────────────────────────┐
│                    Rust 后端（axum）                      │
│                                                          │
│   api/          HTTP 接口层（路由 + handler + 中间件）     │
│   ── 成员 B 负责 ──                                      │
│        │ 调用                                            │
│   services/     业务逻辑层                                │
│   ── 成员 A 负责 ──                                      │
│        │ 读写                                            │
│   db/          数据库层（连接池 + 建表）                  │
│   models/      数据模型（struct 定义）                    │
│   ── 成员 A 负责 ──                                      │
│        │                                                 │
│   SQLite 数据库（rdms.db）                               │
└──────────────────────────────────────────────────────────┘
```

分层职责：

- **models（数据模型）**：定义实体结构体，是「数据库表结构」与「JSON 接口结构」的单一事实来源；
- **services（业务逻辑）**：封装 SQL 与业务规则，暴露纯函数供上层调用；
- **api（接口层）**：解析 HTTP 请求 → 调用 service → 返回 JSON；不含业务细节；
- **frontend（前端）**：通过 `fetch` 调用 `/api/*` 接口渲染页面。

---

## 四、目录结构

```
rdms/
├── Cargo.toml                 # 项目依赖配置        【A】
├── .gitignore                 # 忽略 target、*.db   【A】
├── README.md                  # 本文件              【A】
├── src/                       # Rust 后端
│   ├── main.rs                # 程序入口            【A】
│   ├── lib.rs                 # 模块声明            【A】
│   ├── config.rs              # 配置加载            【A】
│   ├── error.rs               # 统一错误类型        【B】
│   ├── models/                # 数据模型            【A】
│   │   ├── mod.rs
│   │   ├── user.rs            #   成员模型
│   │   ├── project.rs         #   项目模型
│   │   ├── task.rs            #   任务模型
│   │   ├── attendance.rs      #   工时模型
│   │   └── budget.rs          #   预算/成本模型
│   ├── db/                    # 数据库层            【A】
│   │   ├── mod.rs             #   连接池与迁移
│   │   └── migrations.rs      #   建表 SQL
│   ├── services/              # 业务逻辑层          【A】
│   │   ├── mod.rs
│   │   ├── user_service.rs
│   │   ├── project_service.rs
│   │   ├── task_service.rs
│   │   ├── attendance_service.rs
│   │   └── budget_service.rs
│   └── api/                   # HTTP 接口层         【B】
│       ├── mod.rs
│       ├── router.rs          #   路由定义
│       ├── handlers/          #   请求处理器
│       │   ├── mod.rs
│       │   ├── user_handler.rs
│       │   ├── project_handler.rs
│       │   ├── task_handler.rs
│       │   ├── attendance_handler.rs
│       │   └── budget_handler.rs
│       └── middleware/        #   中间件
│           ├── mod.rs
│           └── auth.rs
└── frontend/                  # 前端界面            【C】
    ├── index.html
    ├── css/
    │   └── style.css
    └── js/
        ├── api.js             #   API 封装
        ├── router.js          #   前端路由
        ├── main.js            #   入口
        └── views/             #   各页面视图
            ├── dashboard.js
            ├── members.js
            ├── projects.js
            ├── tasks.js
            ├── attendance.js
            └── budget.js
```

---

## 五、三人分工（重要）

> 建议成员 A 担任**组长**，负责搭建项目骨架、统一数据模型与接口约定，供 B、C 并行开发。

### 成员 A —— Rust 核心逻辑与数据层（组长）

负责 **models / db / services** 三大核心层，以及项目入口与配置。

**负责编写的文件：**

| 文件 | 内容 |
| --- | --- |
| `Cargo.toml` | 项目依赖配置 |
| `.gitignore` | 忽略规则 |
| `README.md` | 架构说明与分工（协作维护） |
| `src/main.rs` | 程序入口：初始化、启动服务 |
| `src/lib.rs` | 模块声明 |
| `src/config.rs` | 配置加载 |
| `src/models/mod.rs` | 模型模块汇总与 re-export |
| `src/models/user.rs` | 成员模型 |
| `src/models/project.rs` | 项目模型 |
| `src/models/task.rs` | 任务模型 |
| `src/models/attendance.rs` | 工时模型 |
| `src/models/budget.rs` | 预算/成本模型 |
| `src/db/mod.rs` | 数据库连接池与迁移 |
| `src/db/migrations.rs` | 建表 SQL |
| `src/services/mod.rs` | 业务逻辑模块汇总 |
| `src/services/user_service.rs` | 成员业务逻辑 |
| `src/services/project_service.rs` | 项目业务逻辑 |
| `src/services/task_service.rs` | 任务业务逻辑 |
| `src/services/attendance_service.rs` | 工时业务逻辑 |
| `src/services/budget_service.rs` | 预算/成本与经济决策逻辑 |

### 成员 B —— Rust HTTP 接口层

负责 **api / error** 层，把 A 的业务逻辑暴露为 REST 接口。

**负责编写的文件：**

| 文件 | 内容 |
| --- | --- |
| `src/error.rs` | 统一错误类型与响应转换 |
| `src/api/mod.rs` | 接口层模块汇总 |
| `src/api/router.rs` | 路由定义与静态文件服务 |
| `src/api/handlers/mod.rs` | 处理器模块汇总 |
| `src/api/handlers/user_handler.rs` | 成员接口 |
| `src/api/handlers/project_handler.rs` | 项目接口（含成本核算） |
| `src/api/handlers/task_handler.rs` | 任务接口 |
| `src/api/handlers/attendance_handler.rs` | 工时接口 |
| `src/api/handlers/budget_handler.rs` | 预算/成本接口 |
| `src/api/middleware/mod.rs` | 中间件汇总 |
| `src/api/middleware/auth.rs` | CORS / 日志 / 鉴权扩展点 |

### 成员 C —— 用户交互界面（前端）

负责 **frontend/** 全部内容，调用 B 提供的接口渲染页面。

**负责编写的文件：**

| 文件 | 内容 |
| --- | --- |
| `frontend/index.html` | 页面骨架与导航 |
| `frontend/css/style.css` | 全局样式 |
| `frontend/js/api.js` | API 封装（fetch） |
| `frontend/js/router.js` | 前端路由 |
| `frontend/js/main.js` | 入口 |
| `frontend/js/views/dashboard.js` | 工作台 |
| `frontend/js/views/members.js` | 成员管理 |
| `frontend/js/views/projects.js` | 项目管理 |
| `frontend/js/views/tasks.js` | 任务管理 |
| `frontend/js/views/attendance.js` | 工时管理 |
| `frontend/js/views/budget.js` | 成本与决策 |

---

## 六、协作接口约定（三人并行开发的“契约”）

为避免集成时返工，A、B、C 在开工前共同确认以下约定：

### 6.1 数据模型（A 定义，B/C 遵循）

- 字段命名、类型、是否必填，统一以 `src/models/*.rs` 中的 struct 为准；
- JSON 序列化规则已通过 `#[serde(rename_all = "lowercase")]` 固定（枚举值一律小写）。

### 6.2 REST 接口（B 实现，C 对接）

| 方法 | 路径 | 说明 |
| --- | --- | --- |
| GET | `/api/users` | 成员列表 |
| POST | `/api/users` | 新增成员 |
| GET | `/api/users/:id` | 成员详情 |
| PUT | `/api/users/:id` | 更新成员 |
| DELETE | `/api/users/:id` | 删除成员 |
| GET/POST | `/api/projects` | 项目列表 / 新增 |
| GET/PUT/DELETE | `/api/projects/:id` | 项目详情 / 更新 / 删除 |
| GET | `/api/projects/:id/cost-summary` | 成本核算（经济决策） |
| GET/POST | `/api/tasks?project_id=` | 任务列表（可按项目过滤）/ 新增 |
| GET/PUT/DELETE | `/api/tasks/:id` | 任务详情 / 更新 / 删除 |
| GET/POST | `/api/attendance?user_id=&task_id=` | 工时列表 / 填报 |
| GET/POST | `/api/budgets?project_id=` | 成本明细 / 录入 |

统一约定：

- 请求体 / 响应体均为 `application/json`；
- 错误响应统一为 `{ "error": "错误描述" }`；
- 成功删除返回 204 或 `{ "ok": true }`。

### 6.3 服务层函数签名（A 提供，B 调用）

B 只需调用 `services/*` 中暴露的函数（第一个参数为 `&SqlitePool`），无需编写 SQL，具体签名见各 `*_service.rs` 文件。

---

## 七、本地运行

```bash
# 1. 进入项目目录
cd rdms

# 2. 编译（首次会拉取依赖）
cargo build

# 3. 运行
cargo run

# 4. 浏览器访问
# http://127.0.0.1:8080
```

> 首次运行会自动创建 `rdms.db` 数据库并建表。

---

## 八、开发建议与里程碑

1. **第 1 周（A 先行）**：A 完成 `Cargo.toml`、`models`、`db` 骨架，确认可编译；
2. **第 2 周（并行）**：B 按服务层签名实现 `api`；C 按接口约定实现 `frontend`；
3. **第 3 周（集成）**：三方联调，跑通「成员 → 项目 → 任务 → 工时 → 成本核算」完整流程；
4. **第 4 周（报告）**：补充测试数据、截图，撰写报告并对照课程三项要求。

---

## 九、课程要求对照（报告素材）

- **① 计算机系统应用环境与开发工具的熟练掌握**：使用 Rust + cargo + git 完成构建与版本协作；axum/sqlx 等工具链的运用。
- **② 多学科背景下的团队合作**：计科（后端逻辑）与大禹（工程/经济视角的模块设计）分工协作，通过接口契约并行开发、git 协作。
- **③ 工程管理原理与经济决策方法的应用**：项目管理（进度/里程碑）、任务分配体现工程管理；成本核算、预算执行率、ROI 体现经济决策方法。
