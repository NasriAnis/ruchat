/* ============================================================
   Login / Register modal
   ============================================================ */
function openAuthModal(mode) {
    // mode: 'login' or 'register'
    const overlay = document.createElement('div');
    overlay.classList.add('popup-overlay');
    overlay.id = 'authOverlay';

    const box = document.createElement('div');
    box.classList.add('popup-box');

    const closeBtn = document.createElement('button');
    closeBtn.classList.add('popup-close');
    closeBtn.textContent = '\u2715';
    closeBtn.addEventListener('click', () => overlay.remove());

    const title = document.createElement('h2');
    title.textContent = mode === 'login' ? 'Login' : 'Register';

    const form = document.createElement('form');
    form.classList.add('auth-form');
    form.innerHTML = `
        <label for="authUsername">Username:</label>
        <input type="text" id="authUsername" name="username" autocomplete="username">
        <label for="authPassword">Password:</label>
        <input type="password" id="authPassword" name="password" autocomplete="${mode === 'login' ? 'current-password' : 'new-password'}">
        <span id="authStatusMsg"></span>
        <button type="submit">${mode === 'login' ? 'Login' : 'Register'}</button>
    `;

    const switchLine = document.createElement('div');
    switchLine.classList.add('switch-auth');
    if (mode === 'login') {
        switchLine.innerHTML = `Don't have an account? <a onclick="switchAuthMode('register')">Register</a>`;
    } else {
        switchLine.innerHTML = `Already have an account? <a onclick="switchAuthMode('login')">Login</a>`;
    }

    box.appendChild(closeBtn);
    box.appendChild(title);
    box.appendChild(form);
    box.appendChild(switchLine);
    overlay.appendChild(box);
    document.body.appendChild(overlay);

    form.addEventListener('submit', function(event) {
        event.preventDefault();
        if (mode === 'login') {
            try_login();
        } else {
            try_register();
        }
    });

    // close modal when clicking outside the box
    overlay.addEventListener('click', (e) => {
        if (e.target === overlay) overlay.remove();
    });
}

function switchAuthMode(mode) {
    const overlay = document.getElementById('authOverlay');
    if (overlay) overlay.remove();
    openAuthModal(mode);
}

function showAuthStatus(success, message) {
    const statusMsg = document.getElementById('authStatusMsg');
    if (!statusMsg) return;
    statusMsg.textContent = message;
    statusMsg.className = success ? 'success' : 'error';
}

async function try_login() {
    const username = document.getElementById('authUsername').value;
    const password = document.getElementById('authPassword').value;
    const url = window.location.origin + "/api/login";
    try {
        const response = await fetch(url, {
            method: "POST",
            body: JSON.stringify({
                "username": username,
                "password": password
            }),
        });
        if (response.ok) {
            const cookie = await response.headers.get("Cookie");
            document.cookie = cookie;
            showAuthStatus(true, "Login successful");
            checkAuth();
	    setTimeout(() => {
		location.reload();
	    }, 600);
        } else {
            showAuthStatus(false, "Login failed (" + response.status + ")");
        }
    } catch (error) {
        console.error(error);
        showAuthStatus(false, "Could not reach server");
    }
    document.getElementById('authUsername').value = '';
    document.getElementById('authPassword').value = '';
}

async function try_register() {
    const username = document.getElementById('authUsername').value;
    const password = document.getElementById('authPassword').value;
    const url = window.location.origin + "/api/register";
    try {
        const response = await fetch(url, {
            method: "POST",
            body: JSON.stringify({
                "username": username,
                "password": password
            }),
        });
        if (await response.ok) {
            const cookie = await response.headers.get("Cookie");
            document.cookie = cookie;
            showAuthStatus(true, "Registration successful");
            checkAuth();
	    setTimeout(() => {
		location.reload();
	    }, 600);
	} else {
            console.log("ERROR");
            showAuthStatus(false, "Registration failed (" + response.status + ")");
        }
    } catch (error) {
        console.error(error);
        showAuthStatus(false, "Could not reach server");
    }
    document.getElementById('authUsername').value = '';
    document.getElementById('authPassword').value = '';
}
