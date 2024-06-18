let username;
let ws;

function enterChat() {
    username = document.getElementById('usernameInput').value;
    if (username) {
        document.getElementById('login').style.display = 'none';
        document.getElementById('chat').style.display = 'block';
        ws = new WebSocket('ws://192.168.1.X:5000/ws/');  // cambiaremos la IP según sea necesario
        ws.onopen = function() {
            console.log('Conexión establecida');
        };
        ws.onmessage = function(event) {
            console.log('Mensaje recibido:', event.data);
            // si el mensaje fue enviado por el usuario actual
            const isSent = event.data.startsWith(username + ": ");
            displayMessage(event.data, isSent);
        };
    } else {
        alert("Please enter a username.");
    }
}

function sendMessage() {
    const input = document.getElementById('messageInput');
    const message = input.value;
    if (message) {
        ws.send(message);
        displayMessage(username + ": " + message, true);
        input.value = '';
    }
}

function displayMessage(message, isSent) {
    const messages = document.getElementById('messages');
    const messageDiv = document.createElement('div');
    messageDiv.classList.add('message');
    messageDiv.textContent = message;

    if (isSent) {
        messageDiv.classList.add('sent');
    } else {
        messageDiv.classList.add('received');
    }

    messages.appendChild(messageDiv);
    messages.scrollTop = messages.scrollHeight; // Scroll 
}

document.getElementById('messageInput').addEventListener('keypress', function(e) {
    if (e.key === 'Enter') {
        sendMessage();
    }
});

