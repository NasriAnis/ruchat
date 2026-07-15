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

function showPopup(message) {
    const overlay = document.createElement('div');
    overlay.classList.add('popup-overlay');

    const box = document.createElement('div');
    box.classList.add('popup-box');

    const text = document.createElement('p');
    text.textContent = message;

    const okBtn = document.createElement('button');
    okBtn.textContent = 'OK';
    okBtn.addEventListener('click', () => {
        overlay.remove();
    });

    box.appendChild(text);
    box.appendChild(okBtn);
    overlay.appendChild(box);
    document.body.appendChild(overlay);
}

function handleUserInput() {
    const message = {
        message: userInput.value.trim(),
    };
    websocket.send(JSON.stringify(message));
    userInput.value = '';
}

sendBtn.addEventListener('click', handleUserInput);

userInput.addEventListener('keydown', (e) => {
    if (e.key === 'Enter') handleUserInput();
});

websocket.addEventListener("error", (e) => {
    console.log(e);
});

websocket.addEventListener("close", (e) => {
    console.log("Websocket Closed:", e.code, e.reason);
    if (e.reason) {
	showPopup(e.reason);
    }
});

websocket.addEventListener("message", (e) => {
    const data = JSON.parse(e.data);
    if (data.username == "You"){
	addMessage(data.message, 'user');
    } else {
	addMessage(data.message, 'bot');
    }
});
