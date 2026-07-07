document.getElementById('login form').addEventListener('submit', function(event) {
    event.preventDefault(); // Prevent form submission
    var username = document.getElementById('username').value;
    var password = document.getElementById('password').value;

    const url = window.location.origin + "/api/login";
    const response = fetch(url, {
	method: "POST",
	body: JSON.stringify({
	    "username": username,
	    "password": password
	}),
    });
});
