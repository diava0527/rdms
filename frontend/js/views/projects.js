import { projectApi } from '../api.js';
import { showToast, openModal, closeModal } from '../main.js';

export async function renderProjects() {
    const app = document.getElementById('app');

    try {
        const projects = await projectApi.list();

        app.innerHTML = `
            <div class="page-header">
                <h2>📁 项目管理</h2>
                <button class="btn btn-primary" onclick="window.openCreateProjectModal()">+ 新增项目</button>
            </div>

            <div class="card">
                <div class="table-wrapper">
                    <table>
                        <thead>
                            <tr>
                                <th>ID</th>
                                <th>项目名称</th>
                                <th>描述</th>
                                <th>预算</th>
                                <th style="text-align:center;">操作</th>
                            </tr>
                        </thead>
                        <tbody>
                            ${projects.length === 0 ? `
                                <tr><td colspan="5" class="empty-state">暂无项目</td></tr>
                            ` : projects.map(p => `
                                <tr>
                                    <td>${p.id}</td>
                                    <td><strong>${p.name}</strong></td>
                                    <td>${p.description || '-'}</td>
                                    <td>¥${p.budget || 0}</td>
                                    <td style="text-align:center;">
                                        <button class="btn btn-primary btn-sm" onclick="window.openEditProjectModal(${p.id})">编辑</button>
                                        <button class="btn btn-warning btn-sm" onclick="window.viewCostSummary(${p.id})">成本</button>
                                        <button class="btn btn-danger btn-sm" onclick="window.deleteProject(${p.id})">删除</button>
                                    </td>
                                </tr>
                            `).join('')}
                        </tbody>
                    </table>
                </div>
            </div>
        `;

        window.openCreateProjectModal = () => openProjectModal(null);
        window.openEditProjectModal = (id) => {
            const p = projects.find(x => x.id === id);
            if (p) openProjectModal(p);
        };

        window.viewCostSummary = async (id) => {
            try {
                const data = await projectApi.costSummary(id);
                openModal(`
                    <h3 class="modal-title">💰 成本核算 - ID:${id}</h3>
                    <div class="card" style="margin:1rem 0;">
                        <p><strong>总成本：</strong>¥${data.total_cost || 0}</p>
                        <p><strong>预算执行率：</strong>${data.execution_rate || 0}%</p>
                        <p><strong>ROI：</strong>${data.roi || 0}%</p>
                        <p><strong>详细：</strong></p>
                        <pre style="background:#f1f5f9;padding:0.5rem;border-radius:4px;">${JSON.stringify(data.details || {}, null, 2)}</pre>
                    </div>
                    <div class="modal-footer">
                        <button class="btn btn-primary" onclick="closeModal()">关闭</button>
                    </div>
                `);
            } catch (err) {
                showToast('❌ 加载成本失败: ' + err.message, 'error');
            }
        };

        window.deleteProject = async (id) => {
            if (!confirm('确认删除该项目吗？')) return;
            try {
                await projectApi.delete(id);
                showToast('✅ 删除成功', 'success');
                renderProjects();
            } catch (err) {
                showToast('❌ 删除失败: ' + err.message, 'error');
            }
        };

    } catch (err) {
        app.innerHTML = `<div class="card" style="color:#ef4444;">加载失败：${err.message}</div>`;
    }
}

function openProjectModal(project = null) {
    const isEdit = !!project;

    openModal(`
        <h3 class="modal-title">${isEdit ? '✏️ 编辑项目' : '➕ 新增项目'}</h3>
        <form id="project-form">
            <div class="form-group">
                <label>项目名称 *</label>
                <input type="text" id="f-name" value="${project?.name || ''}" required />
            </div>
            <div class="form-group">
                <label>描述</label>
                <textarea id="f-desc">${project?.description || ''}</textarea>
            </div>
            <div class="form-group">
                <label>预算 (¥)</label>
                <input type="number" id="f-budget" value="${project?.budget || 0}" step="0.01" />
            </div>
            <div class="modal-footer">
                <button type="button" class="btn btn-outline" onclick="closeModal()">取消</button>
                <button type="submit" class="btn btn-primary">${isEdit ? '保存' : '创建'}</button>
            </div>
        </form>
    `);

    document.getElementById('project-form').addEventListener('submit', async (e) => {
        e.preventDefault();
        const data = {
            name: document.getElementById('f-name').value.trim(),
            description: document.getElementById('f-desc').value.trim(),
            budget: parseFloat(document.getElementById('f-budget').value) || 0,
        };

        if (!data.name) {
            showToast('请填写项目名称', 'error');
            return;
        }

        try {
            if (isEdit) {
                await projectApi.update(project.id, data);
                showToast('✅ 更新成功', 'success');
            } else {
                await projectApi.create(data);
                showToast('✅ 创建成功', 'success');
            }
            closeModal();
            renderProjects();
        } catch (err) {
            showToast('❌ 操作失败: ' + err.message, 'error');
        }
    });
}
