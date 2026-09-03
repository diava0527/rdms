// 工作台视图：展示概览数据（成员数、项目数、任务数等）。
// 归属：成员 C

function renderDashboard(container) {
    // TODO(成员 C)：调用 api 拉取概览数据并渲染
    container.innerHTML = `
        <h2>工作台</h2>
        <div class="card">
            <p>这里展示：成员数量、进行中项目、待办任务、预算执行情况等概览卡片。</p>
        </div>
    `;
}
