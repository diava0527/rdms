import { taskApi, projectApi, userApi } from '../api.js';
import { showToast, openModal, closeModal } from '../main.js';

let allProjects = [];
let allUsers = [];

export async function renderTasks() {
    const app = document.getElementById('app');

    try {
        const [tasks, projects, users] = await Promise.all([
            taskApi.list(),
            projectApi.list(),
            userApi.list(),
        ]);
        allProjects = projects;
        allUsers = users;

        app.innerHTML = `
            <div class="page-header">
                <h2>✅ 任务管理</h2>
                <button class="btn btn-primary" onclick="window.openCreateTaskModal()">+ 新增任务</button>
            </div>

            <div class="card">
                <div class="table-wrapper">
                    <table>
                        <thead>
                            <tr>
                                <th>ID</th>
                                <th>任务名称</th>
                                <th>项目</th>
                                <th>负责人</th>
                                <th>状态</th>
                                <th style="text-align:center;">操作</th>
                            </tr>
                        </thead>
                        <tbody>
                            ${tasks.length === 0 ? `
                                <tr><td colspan="6" class="empty-state">暂无任务</td></tr>
                            ` : tasks.map(t => {
                                const project = projects.find(p => p.id === t.project_id);
                                const user = users.find(u => u.id === t.assignee_id);
                                return `
                                    <tr>
                                        <td>${t.id}</td>
                                        <td><strong>${t.title || t.name}</strong></td>
                                        <td>${project?.name || '-'}</td>
                                        <td>${user?.name || '未分配'}</td>
                                        <td><span class="badge badge-${t.status === 'done' ? 'green' : t.status === 'doing' ? 'yellow' : 'gray'}">${t.status || '待处理'}</span></td>
                                        <td style="text-align:center;">
                                            <button class="btn btn-primary btn-sm" onclick="window.openEditTaskModal(${t.id})">编辑</button>
                                            <button class="btn btn-danger btn-sm" onclick="window.deleteTask(${t.id})">删除</button>
                                        </td>
                                    </tr>
                                `;
                            }).join('')}
                        </tbody>
                    </table>
                </div>
            </div>
        `;

        window.openCreateTaskModal = () => openTaskModal(null);
        window.openEditTaskModal = (id) => {
            const t = tasks.find(x => x.id === id);
            if (t) openTaskModal(t);
        };

        window.deleteTask = async (id) => {
            if (!confirm('确认删除该任务吗？')) return;
            try {
                await taskApi.delete(id);
                showToast('✅ 删除成功', 'success');
                renderTasks();
            } catch (err) {
                showToast('❌ 删除失败: ' + err.message, 'error');
            }
        };

    } catch (err) {
        app.innerHTML = `<div class="card" style="color:#ef4444;">加载失败：${err.message}</div>`;
    }
}

function openTaskModal(task = null) {
    const isEdit = !!task;

    openModal(`
        <h3 class="modal-title">${isEdit ? '✏️ 编辑任务' : '➕ 新增任务'}</h3>
        <form id="task-form">
            <div class="form-group">
                <label>任务名称 *</label>
                <input type="text" id="f-name" value="${task?.title || task?.name || ''}" required />
            </div>
            <div class="form-row">
                <div class="form-group">
                    <label>所属项目</label>
                    <select id="f-project">
                        <option value="">无</option>
                        ${allProjects.map(p => `
                            <option value="${p.id}" ${task?.project_id === p.id ? 'selected' : ''}>${p.name}</option>
                        `).join('')}
                    </select>
                </div>
                <div class="form-group">
                    <label>负责人</label>
                    <select id="f-user">
                        <option value="">未分配</option>
                        ${allUsers.map(u => `
                            <option value="${u.id}" ${task?.assignee_id === u.id ? 'selected' : ''}>${u.name}</option>
                        `).join('')}
                    </select>
                </div>
            </div>
            <div class="form-group">
                <label>状态</label>
                <select id="f-status">
                    <option value="todo" ${task?.status === 'todo' ? 'selected' : ''}>待处理</option>
                    <option value="doing" ${task?.status === 'doing' ? 'selected' : ''}>进行中</option>
                    <option value="done" ${task?.status === 'done' ? 'selected' : ''}>已完成</option>
                </select>
            </div>
            <div class="modal-footer">
                <button type="button" class="btn btn-outline" onclick="closeModal()">取消</button>
                <button type="submit" class="btn btn-primary">${isEdit ? '保存' : '创建'}</button>
            </div>
        </form>
    `);

    document.getElementById('task-form').addEventListener('submit', async (e) => {
        e.preventDefault();
        const data = {
            title: document.getElementById('f-name').value.trim(),
            project_id: parseInt(document.getElementById('f-project').value) || null,
            assignee_id: parseInt(document.getElementById('f-user').value) || null,
            status: document.getElementById('f-status').value,
        };

        if (!data.title) {
            showToast('请填写任务名称', 'error');
            return;
        }

        try {
            if (isEdit) {
                await taskApi.update(task.id, data);
                showToast('✅ 更新成功', 'success');
            } else {
                await taskApi.create(data);
                showToast('✅ 创建成功', 'success');
            }
            closeModal();
            renderTasks();
        } catch (err) {
            showToast('❌ 操作失败: ' + err.message, 'error');
        }
    });
}
