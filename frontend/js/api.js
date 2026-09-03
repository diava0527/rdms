// API 封装：统一封装 fetch 调用，供各视图使用。
// 归属：成员 C

const BASE_URL = "/api";

/**
 * 通用请求函数
 * @param {string} path  接口路径，如 "/users"
 * @param {object} opts  { method, query, body }
 * @returns {Promise<any>} 解析后的 JSON
 */
async function request(path, opts = {}) {
    const { method = "GET", query, body } = opts;

    // 拼接查询参数
    let url = BASE_URL + path;
    if (query) {
        const qs = new URLSearchParams();
        Object.entries(query).forEach(([k, v]) => {
            if (v !== undefined && v !== null) qs.set(k, v);
        });
        url += "?" + qs.toString();
    }

    const res = await fetch(url, {
        method,
        headers: { "Content-Type": "application/json" },
        body: body ? JSON.stringify(body) : undefined,
    });

    if (!res.ok) {
        const err = await res.json().catch(() => ({}));
        throw new Error(err.error || `请求失败: ${res.status}`);
    }

    return res.status === 204 ? null : res.json();
}

// —— 各资源的便捷方法（成员 C 实现）——
const api = {
    // 成员
    users: {
        list: () => request("/users"),
        create: (body) => request("/users", { method: "POST", body }),
        get: (id) => request(`/users/${id}`),
        update: (id, body) => request(`/users/${id}`, { method: "PUT", body }),
        remove: (id) => request(`/users/${id}`, { method: "DELETE" }),
    },
    // 项目
    projects: {
        list: () => request("/projects"),
        create: (body) => request("/projects", { method: "POST", body }),
        update: (id, body) => request(`/projects/${id}`, { method: "PUT", body }),
        remove: (id) => request(`/projects/${id}`, { method: "DELETE" }),
        costSummary: (id) => request(`/projects/${id}/cost-summary`),
    },
    // 任务
    tasks: {
        list: (projectId) => request("/tasks", { query: { project_id: projectId } }),
        create: (body) => request("/tasks", { method: "POST", body }),
        update: (id, body) => request(`/tasks/${id}`, { method: "PUT", body }),
        remove: (id) => request(`/tasks/${id}`, { method: "DELETE" }),
    },
    // 工时
    attendance: {
        list: (query) => request("/attendance", { query }),
        create: (body) => request("/attendance", { method: "POST", body }),
    },
    // 预算/成本
    budgets: {
        list: (projectId) => request("/budgets", { query: { project_id: projectId } }),
        create: (body) => request("/budgets", { method: "POST", body }),
    },
};

// TODO(成员 C)：如使用 ES 模块，可改为 `export default api;`
