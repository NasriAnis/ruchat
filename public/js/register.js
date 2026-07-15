document.getElementById('register-form').addEventListener('submit', function(event) {
    event.preventDefault();
    try_register();
});

function showStatus(success, message) {
    const statusMsg = document.getElementById('statusMsg');
    statusMsg.textContent = message;
    statusMsg.className = success ? 'success' : 'error';
}

async function try_register() {
    var username = document.getElementById('username').value;
    var password = document.getElementById('password').value;
    const url = window.location.origin + "/api/register";

    try{
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
	    showStatus(true, "Registration successful");
	    // console.log(cookie);
	}
	else {
	    console.log("ERROR");
	    showStatus(false, "Registration failed (" + response.status + ")");
	}

    } catch (error) {
	console.error(error);
	showStatus(false, "Could not reach server");
    }
    document.getElementById('username').value = '';
    document.getElementById('password').value = '';
}
