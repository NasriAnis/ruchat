const wsurl = "ws://" + window.location.hostname + ":2121"
const websocket = new WebSocket(wsurl);

const chatBox = document.getElementById('chatBox');
const userInput = document.getElementById('userInput');
const sendBtn = document.getElementById('sendBtn');

function addMessage(message, sender) {
    const messageDiv = document.createElement('div');
    messageDiv.classList.add('message', sender);
    messageDiv.textContent = message;
    chatBox.appendChild(messageDiv);
    chatBox.scrollTop = chatBox.scrollHeight;
}

function getCookie(name) {
  const value = `; ${document.cookie}`;
  const parts = value.split(`; ${name}=`);
  if (parts.length === 2) return parts.pop().split(';').shift();
}


function handleUserInput() {
    const message = {
        message: userInput.value.trim(),
	cookie: getCookie("authToken")
    };
    websocket.send(JSON.stringify(message));
    userInput.value = '';
}

sendBtn.addEventListener('click', handleUserInput);

userInput.addEventListener('keydown', (e) => {
    if (e.key === 'Enter') handleUserInput();
});

websocket.addEventListener("error", (e) => {
    console.log(`ERROR`);
});

websocket.addEventListener("message", (e) => {
    const data = JSON.parse(e.data);
    addMessage(data.message, 'bot');
});
