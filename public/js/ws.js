/* ============================================================
   Chat / websocket
   ============================================================ */
const wsurl = "wss://" + window.location.hostname + ":2121"
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
