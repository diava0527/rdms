import { userApi, projectApi, taskApi, attendanceApi } from '../api.js';

export async function renderDashboard() {
    const app = document.getElementById('app');

    try {
        const [users, projects, tasks, attendances] = await Promise.all([
            userApi.list(),
            projectApi.list(),
            taskApi.list(),
            attendanceApi.list(),
        ]);

        const totalHours = attendances.reduce((sum, a) => sum + (a.hours || 0), 0);

        app.innerHTML = `
            <div class="page-header">
                <h2>📈 工作台</h2>
                <span class="text-muted">概览</span>
            </div>

            <div class="stats-grid">
                <div class="stat-card">
                    <div class="stat-icon">👥</div>
                    <div class="stat-label">成员总数</div>
                    <div class="stat-value">${users.length}</div>
                </div>
                <div class="stat-card">
                    <div class="stat-icon">📁</div>
                    <div class="stat-label">项目总数</div>
                    <div class="stat-value">${projects.length}</div>
                </div>
                <div class="stat-card">
                    <div class="stat-icon">✅</div>
                    <div class="stat-label">任务总数</div>
                    <div class="stat-value">${tasks.length}</div>
                </div>
                <div class="stat-card">
                    <div class="stat-icon">⏱️</div>
                    <div class="stat-label">总工时</div>
                    <div class="stat-value">${totalHours}h</div>
                </div>
            </div>

            <div style="display:grid; grid-template-columns: 1fr 1fr; gap:1.5rem;">
                <div class="card">
                    <h3 style="margin-bottom:0.75rem;">📋 最近任务</h3>
                    ${tasks.slice(0, 5).map(t => `
                        <div style="padding:0.5rem 0; border-bottom:1px solid #f1f5f9; display:flex; justify-content:space-between;">
                            <span>${t.title || t.name || '未命名任务'}</span>
                            <span class="badge badge-${t.status === 'done' ? 'green' : t.status === 'doing' ? 'yellow' : 'gray'}">${t.status || '待处理'}</span>
                        </div>
                    `).join('') || '<div class="text-muted">暂无任务</div>'}
                </div>

                <div class="card">
                    <h3 style="margin-bottom:0.75rem;">📊 活跃项目</h3>
                    ${projects.slice(0, 5).map(p => `
                        <div style="padding:0.5rem 0; border-bottom:1px solid #f1f5f9; display:flex; justify-content:space-between;">
                            <span>${p.name}</span>
                            <span class="text-muted text-sm">预算 ¥${p.budget || 0}</span>
                        </div>
                    `).join('') || '<div class="text-muted">暂无项目</div>'}
                </div>
            </div>
        `;
    } catch (err) {
        app.innerHTML = `
            <div class="card" style="text-align:center; padding:2rem; color:#ef4444;">
                <h3>⚠️ 加载失败</h3>
                <p class="text-muted">${err.message}</p>
                <button class="btn btn-primary mt-2" onclick="location.reload()">重新加载</button>
            </div>
        `;
    }
}
