document.getElementById('login-form').addEventListener('submit', function(event) {
    event.preventDefault();
    try_login();
});

function showStatus(success, message) {
    const statusMsg = document.getElementById('statusMsg');
    statusMsg.textContent = message;
    statusMsg.className = success ? 'success' : 'error';
}

async function try_login() {
    var username = document.getElementById('username').value;
    var password = document.getElementById('password').value;
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
            showStatus(true, "Login successful");
        } else {
            showStatus(false, "Login failed (" + response.status + ")");
        }
    } catch (error) {
        console.error(error);
        showStatus(false, "Could not reach server");
    }
    document.getElementById('username').value = '';
    document.getElementById('password').value = '';
}
