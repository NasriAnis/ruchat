/* ============================================================
   Cookie / auth helpers
   ============================================================ */
function getCookie(name) {
    const value = `; ${document.cookie}`;
    const parts = value.split(`; ${name}=`);
    if (parts.length === 2) return parts.pop().split(';').shift();
}

function isLoggedIn() {
    const match = document.cookie
        .split(';')
        .map(c => c.trim())
        .find(c => c.startsWith('authToken='));
    return !!match && match.split('=')[1] !== '';
}

function logout() {
    if (websocket && websocket.readyState === WebSocket.OPEN) {
        websocket.close(1000, "User logged out");
    }
    document.cookie = "authToken=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";
    setTimeout(() => {
        location.reload();
    }, 200);
}

function checkAuth() {
    const authLinks = document.getElementById('authLinks');
    if (isLoggedIn()) {
        authLinks.innerHTML = `<button class="auth-btn" onclick="logout()">Logout</button>`;
    } else {
        authLinks.innerHTML = `
            <button class="auth-btn" onclick="openAuthModal('login')">Login</button>
            <button class="auth-btn" onclick="openAuthModal('register')">Register</button>
        `;
    }
}

window.addEventListener('DOMContentLoaded', checkAuth);
