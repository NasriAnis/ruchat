document.getElementById('register form').addEventListener('submit', function(event) {
    event.preventDefault();
    try_login();
});

async function try_login() {
    var username = document.getElementById('username').value;
    var password = document.getElementById('password').value;
    const url = window.location.origin + "/api/login";

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
	    console.log(cookie);
	}
	else {
	    console.log("ERROR");
	}

    } catch (error) {
	console.error(error);
    }
    
}
