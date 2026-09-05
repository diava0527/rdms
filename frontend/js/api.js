// ============================================================
// API 封装层 —— 统一调用后端 REST 接口
// ============================================================

const BASE_URL = '/api';

/**
 * 通用请求函数
 */
async function request(endpoint, options = {}) {
    const url = `${BASE_URL}${endpoint}`;
    const config = {
        headers: {
            'Content-Type': 'application/json',
            ...options.headers,
        },
        ...options,
    };

    // 如果有 body，自动 JSON 序列化
    if (options.body && typeof options.body === 'object') {
        config.body = JSON.stringify(options.body);
    }

    try {
        const response = await fetch(url, config);
        const data = await response.json().catch(() => ({}));

        if (!response.ok) {
            const errorMsg = data.error || data.message || `HTTP ${response.status}`;
            throw new Error(errorMsg);
        }

        return data;
    } catch (err) {
        // 网络错误也抛出
        if (err instanceof Error) {
            throw err;
        }
        throw new Error('网络请求失败');
    }
}

// ============================================================
// 成员 API
// ============================================================
export const userApi = {
    list: () => request('/users'),
    get: (id) => request(`/users/${id}`),
    create: (data) => request('/users', { method: 'POST', body: data }),
    update: (id, data) => request(`/users/${id}`, { method: 'PUT', body: data }),
    delete: (id) => request(`/users/${id}`, { method: 'DELETE' }),
};

// ============================================================
// 项目 API
// ============================================================
export const projectApi = {
    list: () => request('/projects'),
    get: (id) => request(`/projects/${id}`),
    create: (data) => request('/projects', { method: 'POST', body: data }),
    update: (id, data) => request(`/projects/${id}`, { method: 'PUT', body: data }),
    delete: (id) => request(`/projects/${id}`, { method: 'DELETE' }),
    costSummary: (id) => request(`/projects/${id}/cost-summary`),
};

// ============================================================
// 任务 API
// ============================================================
export const taskApi = {
    list: (projectId) => {
        const query = projectId ? `?project_id=${projectId}` : '';
        return request(`/tasks${query}`);
    },
    get: (id) => request(`/tasks/${id}`),
    create: (data) => request('/tasks', { method: 'POST', body: data }),
    update: (id, data) => request(`/tasks/${id}`, { method: 'PUT', body: data }),
    delete: (id) => request(`/tasks/${id}`, { method: 'DELETE' }),
};

// ============================================================
// 工时 API
// ============================================================
export const attendanceApi = {
    list: (params = {}) => {
        const query = new URLSearchParams(params).toString();
        return request(`/attendance${query ? '?' + query : ''}`);
    },
    create: (data) => request('/attendance', { method: 'POST', body: data }),
    update: (id, data) => request(`/attendance/${id}`, { method: 'PUT', body: data }),
    delete: (id) => request(`/attendance/${id}`, { method: 'DELETE' }),
};

// ============================================================
// 预算/成本 API
// ============================================================
export const budgetApi = {
    list: (projectId) => {
        const query = projectId ? `?project_id=${projectId}` : '';
        return request(`/budgets${query}`);
    },
    create: (data) => request('/budgets', { method: 'POST', body: data }),
    delete: (id) => request(`/budgets/${id}`, { method: 'DELETE' }),
};
