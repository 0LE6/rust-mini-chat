const ws = new WebSocket('ws://127.0.0.1:5000/ws/');

ws.onopen = function() {
    console.log('Conexión establecida');
};

ws.onmessage = function(event) {
    console.log('Mensaje recibido:', event.data);
    displayMessage(event.data, false);
};

function sendMessage() {
    const input = document.getElementById('messageInput');
    const message = input.value;
    console.log('Enviando mensaje:', message);
    ws.send(message);
    displayMessage(message, true);
    input.value = '';
}

function displayMessage(message, isSent) {
    const messages = document.getElementById('messages');
    const messageDiv = document.createElement('div');
    messageDiv.textContent = message;
    messageDiv.className = isSent ? 'sent' : 'received';
    messages.appendChild(messageDiv);
}

