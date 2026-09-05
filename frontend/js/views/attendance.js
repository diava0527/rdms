import { attendanceApi, userApi, taskApi } from '../api.js';
import { showToast, openModal, closeModal } from '../main.js';

let allUsers = [];
let allTasks = [];

export async function renderAttendance() {
    const app = document.getElementById('app');

    try {
        const [attendances, users, tasks] = await Promise.all([
            attendanceApi.list(),
            userApi.list(),
            taskApi.list(),
        ]);
        allUsers = users;
        allTasks = tasks;

        app.innerHTML = `
            <div class="page-header">
                <h2>⏱️ 工时管理</h2>
                <button class="btn btn-primary" onclick="window.openCreateAttendanceModal()">+ 填报工时</button>
            </div>

            <div class="card">
                <div class="table-wrapper">
                    <table>
                        <thead>
                            <tr>
                                <th>ID</th>
                                <th>成员</th>
                                <th>任务</th>
                                <th>工时 (h)</th>
                                <th>日期</th>
                                <th style="text-align:center;">操作</th>
                            </tr>
                        </thead>
                        <tbody>
                            ${attendances.length === 0 ? `
                                <tr><td colspan="6" class="empty-state">暂无工时记录</td></tr>
                            ` : attendances.map(a => {
                                const user = users.find(u => u.id === a.user_id);
                                const task = tasks.find(t => t.id === a.task_id);
                                return `
                                    <tr>
                                        <td>${a.id}</td>
                                        <td>${user?.name || '-'}</td>
                                        <td>${task?.title || task?.name || '-'}</td>
                                        <td><strong>${a.hours || 0}</strong></td>
                                        <td>${a.date || '-'}</td>
                                        <td style="text-align:center;">
                                            <button class="btn btn-danger btn-sm" onclick="window.deleteAttendance(${a.id})">删除</button>
                                        </td>
                                    </tr>
                                `;
                            }).join('')}
                        </tbody>
                    </table>
                </div>
            </div>
        `;

        window.openCreateAttendanceModal = () => openAttendanceModal(null);
        window.deleteAttendance = async (id) => {
            if (!confirm('确认删除该工时记录吗？')) return;
            try {
                await attendanceApi.delete(id);
                showToast('✅ 删除成功', 'success');
                renderAttendance();
            } catch (err) {
                showToast('❌ 删除失败: ' + err.message, 'error');
            }
        };

    } catch (err) {
        app.innerHTML = `<div class="card" style="color:#ef4444;">加载失败：${err.message}</div>`;
    }
}

function openAttendanceModal(attendance = null) {
    const isEdit = !!attendance;

    openModal(`
        <h3 class="modal-title">${isEdit ? '✏️ 编辑工时' : '➕ 填报工时'}</h3>
        <form id="attendance-form">
            <div class="form-row">
                <div class="form-group">
                    <label>成员 *</label>
                    <select id="f-user" required>
                        <option value="">请选择</option>
                        ${allUsers.map(u => `
                            <option value="${u.id}" ${attendance?.user_id === u.id ? 'selected' : ''}>${u.name}</option>
                        `).join('')}
                    </select>
                </div>
                <div class="form-group">
                    <label>任务</label>
                    <select id="f-task">
                        <option value="">无</option>
                        ${allTasks.map(t => `
                            <option value="${t.id}" ${attendance?.task_id === t.id ? 'selected' : ''}>${t.title || t.name}</option>
                        `).join('')}
                    </select>
                </div>
            </div>
            <div class="form-row">
                <div class="form-group">
                    <label>工时 (小时) *</label>
                    <input type="number" id="f-hours" value="${attendance?.hours || 0}" step="0.5" required min="0" />
                </div>
                <div class="form-group">
                    <label>日期</label>
                    <input type="date" id="f-date" value="${attendance?.date || new Date().toISOString().slice(0,10)}" />
                </div>
            </div>
            <div class="modal-footer">
                <button type="button" class="btn btn-outline" onclick="closeModal()">取消</button>
                <button type="submit" class="btn btn-primary">${isEdit ? '保存' : '创建'}</button>
            </div>
        </form>
    `);

    document.getElementById('attendance-form').addEventListener('submit', async (e) => {
        e.preventDefault();
        const data = {
            user_id: parseInt(document.getElementById('f-user').value),
            task_id: parseInt(document.getElementById('f-task').value) || null,
            hours: parseFloat(document.getElementById('f-hours').value) || 0,
            date: document.getElementById('f-date').value,
        };

        if (!data.user_id) {
            showToast('请选择成员', 'error');
            return;
        }
        if (data.hours <= 0) {
            showToast('请输入有效的工时', 'error');
            return;
        }

        try {
            if (isEdit) {
                await attendanceApi.update(attendance.id, data);
                showToast('✅ 更新成功', 'success');
            } else {
                await attendanceApi.create(data);
                showToast('✅ 填报成功', 'success');
            }
            closeModal();
            renderAttendance();
        } catch (err) {
            showToast('❌ 操作失败: ' + err.message, 'error');
        }
    });
}
