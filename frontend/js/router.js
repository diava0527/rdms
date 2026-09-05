import { renderDashboard } from './views/dashboard.js';
import { renderMembers } from './views/members.js';
import { renderProjects } from './views/projects.js';
import { renderTasks } from './views/tasks.js';
import { renderAttendance } from './views/attendance.js';
import { renderBudget } from './views/budget.js';

/**
 * 路由表：hash → 渲染函数
 */
const routes = {
    dashboard: renderDashboard,
    members: renderMembers,
    projects: renderProjects,
    tasks: renderTasks,
    attendance: renderAttendance,
    budget: renderBudget,
};

/**
 * 获取当前路由名称
 */
function getCurrentRoute() {
    const hash = window.location.hash.slice(1);
    return hash || 'dashboard';
}

/**
 * 导航到指定路由
 */
export function navigate(route) {
    if (route && route !== getCurrentRoute()) {
        window.location.hash = route;
        return;
    }
    renderCurrent();
}

/**
 * 渲染当前路由对应的页面
 */
export function renderCurrent() {
    const route = getCurrentRoute();
    const renderFn = routes[route];

    // 高亮导航
    document.querySelectorAll('.nav-links a').forEach(link => {
        link.classList.toggle('active', link.getAttribute('href') === `#${route}`);
    });

    if (renderFn) {
        renderFn().catch(err => {
            console.error('渲染失败:', err);
            document.getElementById('app').innerHTML = `
                <div class="card" style="color:#ef4444; text-align:center; padding:2rem;">
                    <h3>⚠️ 加载失败</h3>
                    <p class="text-muted">${err.message}</p>
                </div>
            `;
        });
    } else {
        document.getElementById('app').innerHTML = `
            <div class="card" style="text-align:center; padding:2rem;">
                <h3>404</h3>
                <p class="text-muted">页面不存在</p>
            </div>
        `;
    }
}

// 监听 hash 变化
window.addEventListener('hashchange', renderCurrent);

// 首次加载时渲染
document.addEventListener('DOMContentLoaded', renderCurrent);
