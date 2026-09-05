/**
 * 应用程序入口
 * - 初始化 Toast 系统
 * - 提供全局工具函数
 * - 挂载路由
 */

import { renderCurrent } from './router.js';

// ============================================================
// Toast 通知系统
// ============================================================
const toastContainer = document.createElement('div');
toastContainer.className = 'toast-container';
document.body.appendChild(toastContainer);

export function showToast(message, type = 'info', duration = 3000) {
    const toast = document.createElement('div');
    toast.className = `toast toast-${type}`;
    toast.textContent = message;

    toastContainer.appendChild(toast);

    setTimeout(() => {
        toast.classList.add('fade-out');
        setTimeout(() => toast.remove(), 300);
    }, duration);
}

// 挂载到全局方便调试
window.showToast = showToast;

// ============================================================
// 模态框系统
// ============================================================
const modalEl = document.getElementById('modal');
const modalBody = document.getElementById('modal-body');
const modalClose = document.querySelector('.modal-close');

export function openModal(html) {
    modalBody.innerHTML = html;
    modalEl.classList.add('show');
    document.body.style.overflow = 'hidden';
}

export function closeModal() {
    modalEl.classList.remove('show');
    document.body.style.overflow = '';
}

modalClose.addEventListener('click', closeModal);
modalEl.addEventListener('click', (e) => {
    if (e.target === modalEl) closeModal();
});

// 按 ESC 关闭
document.addEventListener('keydown', (e) => {
    if (e.key === 'Escape') closeModal();
});

// 挂载到全局
window.openModal = openModal;
window.closeModal = closeModal;

// ============================================================
// 初始化
// ============================================================
renderCurrent();

console.log('✅ RDMS 前端已启动');
