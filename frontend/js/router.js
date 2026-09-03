// 前端路由：基于 hash 的简单路由（#/members -> members 视图）。
// 归属：成员 C

const routes = {
    "": "dashboard",
    "dashboard": "dashboard",
    "members": "members",
    "projects": "projects",
    "tasks": "tasks",
    "attendance": "attendance",
    "budget": "budget",
};

/**
 * 渲染当前路由对应的视图
 */
function renderRoute() {
    const hash = location.hash.replace(/^#\//, "");
    const viewName = routes[hash] || "dashboard";
    const container = document.getElementById("app");

    // 更新侧边栏高亮
    document.querySelectorAll(".sidebar nav a").forEach((a) => {
        a.classList.toggle("active", a.getAttribute("href") === `#/${hash}`);
    });

    // 调用对应视图的渲染函数（约定：views/<name>.js 暴露 render<Name>）
    // TODO(成员 C)：实现各视图渲染函数
    const render = window[`render${capitalize(viewName)}`];
    if (typeof render === "function") {
        render(container);
    } else {
        container.innerHTML = `<p>视图 ${viewName} 未实现</p>`;
    }
}

function capitalize(s) {
    return s.charAt(0).toUpperCase() + s.slice(1);
}

window.addEventListener("hashchange", renderRoute);
