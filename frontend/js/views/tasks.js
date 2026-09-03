// 任务管理视图：看板或列表，支持状态流转。
// 归属：成员 C

function renderTasks(container) {
    // TODO(成员 C)：调用 api.tasks.list() 渲染任务，支持拖拽/下拉变更状态
    container.innerHTML = `
        <h2>任务管理</h2>
        <div class="card">
            <p>这里展示任务看板（待认领/进行中/待评审/已完成），支持新建与状态流转。</p>
        </div>
    `;
}
