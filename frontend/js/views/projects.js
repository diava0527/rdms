// 项目管理视图：列表 + 新增/编辑，展示进度、里程碑、预算。
// 归属：成员 C

function renderProjects(container) {
    // TODO(成员 C)：调用 api.projects.list() 渲染表格
    container.innerHTML = `
        <h2>项目管理</h2>
        <div class="card">
            <p>这里展示项目列表（名称、状态、负责人、里程碑、预算），以及新增/编辑表单。</p>
        </div>
    `;
}
