import { budgetApi, projectApi } from '../api.js';
import { showToast, openModal, closeModal } from '../main.js';

let allProjects = [];

export async function renderBudget() {
    const app = document.getElementById('app');

    try {
        const [budgets, projects] = await Promise.all([
            budgetApi.list(),
            projectApi.list(),
        ]);
        allProjects = projects;

        // 计算各项目成本汇总
        const projectCosts = {};
        budgets.forEach(b => {
            if (!projectCosts[b.project_id]) {
                projectCosts[b.project_id] = { total: 0, items: [] };
            }
            projectCosts[b.project_id].total += b.amount || 0;
            projectCosts[b.project_id].items.push(b);
        });

        app.innerHTML = `
            <div class="page-header">
                <h2>💰 成本与决策</h2>
                <button class="btn btn-primary" onclick="window.openCreateBudgetModal()">+ 录入成本</button>
            </div>

            ${projects.map(p => {
                const cost = projectCosts[p.id] || { total: 0, items: [] };
                const executionRate = p.budget ? ((cost.total / p.budget) * 100).toFixed(1) : 0;
                return `
                    <div class="card" style="margin-bottom:1rem;">
                        <div style="display:flex; justify-content:space-between; align-items:center; flex-wrap:wrap;">
                            <h3>📊 ${p.name}</h3>
                            <div>
                                <span class="badge badge-${executionRate > 100 ? 'red' : executionRate > 80 ? 'yellow' : 'green'}">
                                    预算执行率 ${executionRate}%
                                </span>
                            </div>
                        </div>
                        <div style="display:grid; grid-template-columns: 1fr 1fr 1fr; gap:1rem; margin-top:0.5rem;">
                            <div><span class="text-muted">预算：</span>¥${p.budget || 0}</div>
                            <div><span class="text-muted">已使用：</span>¥${cost.total}</div>
                            <div><span class="text-muted">剩余：</span>¥${(p.budget || 0) - cost.total}</div>
                        </div>
                        ${cost.items.length > 0 ? `
                            <details style="margin-top:0.5rem;">
                                <summary style="cursor:pointer; color:#3b82f6;">查看明细 (${cost.items.length}项)</summary>
                                <div style="margin-top:0.5rem;">
                                    ${cost.items.map(item => `
                                        <div style="display:flex; justify-content:space-between; padding:0.3rem 0; border-bottom:1px solid #f1f5f9;">
                                            <span>${item.description || '无描述'}</span>
                                            <span>¥${item.amount || 0}</span>
                                        </div>
                                    `).join('')}
                                </div>
                            </details>
                        ` : '<div class="text-muted text-sm">暂无成本记录</div>'}
                    </div>
                `;
            }).join('') || '<div class="card text-muted" style="text-align:center;">暂无项目数据，请先创建项目</div>'}
        `;

        window.openCreateBudgetModal = () => openBudgetModal(null);

    } catch (err) {
        app.innerHTML = `<div class="card" style="color:#ef4444;">加载失败：${err.message}</div>`;
    }
}

function openBudgetModal(budget = null) {
    const isEdit = !!budget;

    openModal(`
        <h3 class="modal-title">${isEdit ? '✏️ 编辑成本' : '➕ 录入成本'}</h3>
        <form id="budget-form">
            <div class="form-group">
                <label>所属项目 *</label>
                <select id="f-project" required>
                    <option value="">请选择</option>
                    ${allProjects.map(p => `
                        <option value="${p.id}" ${budget?.project_id === p.id ? 'selected' : ''}>${p.name}</option>
                    `).join('')}
                </select>
            </div>
            <div class="form-group">
                <label>金额 (¥) *</label>
                <input type="number" id="f-amount" value="${budget?.amount || 0}" step="0.01" required min="0" />
            </div>
            <div class="form-group">
                <label>描述</label>
                <input type="text" id="f-desc" value="${budget?.description || ''}" placeholder="如：服务器费用、人力成本" />
            </div>
            <div class="modal-footer">
                <button type="button" class="btn btn-outline" onclick="closeModal()">取消</button>
                <button type="submit" class="btn btn-primary">${isEdit ? '保存' : '创建'}</button>
            </div>
        </form>
    `);

    document.getElementById('budget-form').addEventListener('submit', async (e) => {
        e.preventDefault();
        const data = {
            project_id: parseInt(document.getElementById('f-project').value),
            amount: parseFloat(document.getElementById('f-amount').value) || 0,
            description: document.getElementById('f-desc').value.trim(),
        };

        if (!data.project_id) {
            showToast('请选择项目', 'error');
            return;
        }
        if (data.amount <= 0) {
            showToast('请输入有效的金额', 'error');
            return;
        }

        try {
            if (isEdit) {
                await budgetApi.update(budget.id, data);
                showToast('✅ 更新成功', 'success');
            } else {
                await budgetApi.create(data);
                showToast('✅ 录入成功', 'success');
            }
            closeModal();
            renderBudget();
        } catch (err) {
            showToast('❌ 操作失败: ' + err.message, 'error');
        }
    });
}
