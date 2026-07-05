const websocket = new WebSocket("ws://192.168.1.65:2121");
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
    console.log(`ERROR`);
});

websocket.addEventListener("message", (e) => {
    const data = JSON.parse(e.data);
    addMessage(data.message, 'bot');
});
