// 成员管理视图：列表 + 新增/编辑/删除成员。
// 归属：成员 C

function renderMembers(container) {
    // TODO(成员 C)：调用 api.users.list() 渲染表格，并绑定表单提交
    container.innerHTML = `
        <h2>成员管理</h2>
        <div class="card">
            <p>这里展示成员列表表格（姓名、角色、部门、邮箱），以及新增/编辑表单。</p>
        </div>
    `;
}
