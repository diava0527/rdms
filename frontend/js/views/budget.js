// 成本与决策视图：成本录入、预算执行率、ROI 展示。
// 归属：成员 C

function renderBudget(container) {
    // TODO(成员 C)：调用 api.budgets.list() 与 api.projects.costSummary() 渲染
    container.innerHTML = `
        <h2>成本与决策</h2>
        <div class="card">
            <p>这里展示项目成本明细、预算执行率、投资回报率（ROI）等经济决策指标。</p>
        </div>
    `;
}
