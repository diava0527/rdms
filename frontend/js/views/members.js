import { userApi } from '../api.js';
import { showToast, openModal, closeModal } from '../main.js';

export async function renderMembers() {
    const app = document.getElementById('app');

    try {
        const users = await userApi.list();

        app.innerHTML = `
            <div class="page-header">
                <h2>👥 成员管理</h2>
                <button class="btn btn-primary" onclick="window.openCreateMemberModal()">+ 新增成员</button>
            </div>

            <div class="card">
                <div class="table-wrapper">
                    <table>
                        <thead>
                            <tr>
                                <th>ID</th>
                                <th>姓名</th>
                                <th>角色</th>
                                <th>部门</th>
                                <th style="text-align:center;">操作</th>
                            </tr>
                        </thead>
                        <tbody>
                            ${users.length === 0 ? `
                                <tr><td colspan="5" class="empty-state">暂无成员，点击「新增成员」添加</td></tr>
                            ` : users.map(u => `
                                <tr>
                                    <td>${u.id}</td>
                                    <td><strong>${u.name}</strong></td>
                                    <td><span class="badge badge-blue">${u.role || '未分配'}</span></td>
                                    <td>${u.department || '-'}</td>
                                    <td style="text-align:center;">
                                        <button class="btn btn-primary btn-sm" onclick="window.openEditMemberModal(${u.id})">编辑</button>
                                        <button class="btn btn-danger btn-sm" onclick="window.deleteMember(${u.id})">删除</button>
                                    </td>
                                </tr>
                            `).join('')}
                        </tbody>
                    </table>
                </div>
            </div>
        `;

        // 挂载全局函数
        window.openCreateMemberModal = () => openMemberModal(null);
        window.openEditMemberModal = (id) => {
            const user = users.find(u => u.id === id);
            if (user) openMemberModal(user);
        };

        window.deleteMember = async (id) => {
            if (!confirm('确认删除该成员吗？')) return;
            try {
                await userApi.delete(id);
                showToast('✅ 删除成功', 'success');
                renderMembers();
            } catch (err) {
                showToast('❌ 删除失败: ' + err.message, 'error');
            }
        };

    } catch (err) {
        app.innerHTML = `<div class="card" style="color:#ef4444;">加载失败：${err.message}</div>`;
    }
}

/**
 * 打开成员表单模态框
 */
function openMemberModal(user = null) {
    const isEdit = !!user;

    openModal(`
        <h3 class="modal-title">${isEdit ? '✏️ 编辑成员' : '➕ 新增成员'}</h3>
        <form id="member-form">
            <div class="form-group">
                <label>姓名 *</label>
                <input type="text" id="f-name" value="${user?.name || ''}" required />
            </div>
            <div class="form-row">
                <div class="form-group">
                    <label>角色</label>
                    <input type="text" id="f-role" value="${user?.role || ''}" placeholder="如：开发、测试" />
                </div>
                <div class="form-group">
                    <label>部门</label>
                    <input type="text" id="f-dept" value="${user?.department || ''}" placeholder="如：研发部" />
                </div>
            </div>
            <div class="modal-footer">
                <button type="button" class="btn btn-outline" onclick="closeModal()">取消</button>
                <button type="submit" class="btn btn-primary">${isEdit ? '保存' : '创建'}</button>
            </div>
        </form>
    `);

    const form = document.getElementById('member-form');
    form.addEventListener('submit', async (e) => {
        e.preventDefault();
        const data = {
            name: document.getElementById('f-name').value.trim(),
            role: document.getElementById('f-role').value.trim(),
            department: document.getElementById('f-dept').value.trim(),
        };

        if (!data.name) {
            showToast('请填写姓名', 'error');
            return;
        }

        try {
            if (isEdit) {
                await userApi.update(user.id, data);
                showToast('✅ 更新成功', 'success');
            } else {
                await userApi.create(data);
                showToast('✅ 创建成功', 'success');
            }
            closeModal();
            renderMembers();
        } catch (err) {
            showToast('❌ 操作失败: ' + err.message, 'error');
        }
    });
}
