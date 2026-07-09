function redirect_chat() {
    window.location.href = window.location.origin + "/chat";
}
function redirect_login() {
    window.location.href = window.location.origin + "/login";
}
function redirect_register() {
    window.location.href = window.location.origin + "/register";
}

function checkAuth() {
    const match = document.cookie
        .split(';')
        .map(c => c.trim())
        .find(c => c.startsWith('authToken='));

    const isLoggedIn = !!match && match.split('=')[1] !== '';

    const authLinks = document.getElementById('authLinks');

    if (isLoggedIn) {
        authLinks.innerHTML = `<button class="auth-btn" onclick="logout()">Logout</button>`;
    } else {
        authLinks.innerHTML = `
            <button class="auth-btn" onclick="redirect_login()">Login</button>
            <button class="auth-btn" onclick="redirect_register()">Register</button>
        `;
    }
}

window.addEventListener('DOMContentLoaded', checkAuth);

function logout() {
    document.cookie = "authToken=";
    location.reload();
}
