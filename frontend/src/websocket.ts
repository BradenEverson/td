import { MessageType, ServerResponse, ServerResponseType } from "./messages";

export const socket = new WebSocket("/");

function handleServerResponse(response: ServerResponse) {
  if ("Chat" in response.message) {
    console.log("Chat message");
    let message: string =
      response.message.Chat[0] + ": " + response.message.Chat[1];
    displayMessage(message);
  } else if ("UserJoin" in response.message) {
    let message: string = response.message.UserJoin + " has joined the server";
    displayColoredMessage(message, "#80a4bf");
  } else if ("UserLeave" in response.message) {
    let message: string = response.message.UserLeave + " has disconnected :(";
    displayColoredMessage(message, "#f07269");
  } else if ("StartGame" in response.message) {
    const userName = response.message.StartGame[0]; // Assuming this is the opponent's name
    const opponentName = response.message.StartGame[1]; // Assuming this is the opponent's name
    switchToGameView(userName, opponentName);
  } else {
    console.log(response.message);
  }
}

function switchToGameView(username: string, opponentName: string) {
  const chatContainer = document.getElementById("chat-container");
  if (chatContainer) {
    chatContainer.style.display = "none";
  }

  const canvas = document.createElement("canvas");
  canvas.id = "game-canvas";

  canvas.style.width = "100%";
  canvas.style.height = "100%";
  canvas.style.position = "absolute";
  canvas.style.top = "0";
  canvas.style.left = "0";

  canvas.width = window.innerWidth;
  canvas.height = window.innerHeight;

  document.body.appendChild(canvas);

  const ctx = canvas.getContext("2d");
  if (ctx) {
    ctx.font = `${canvas.width * 0.05}px Arial`;
    ctx.textAlign = "center";

    // Draw user and opponent names
    ctx.fillText(username, canvas.width / 4, canvas.height / 2);
    ctx.fillText(opponentName, (3 * canvas.width) / 4, canvas.height / 2);
  }

  window.addEventListener("resize", () => {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    if (ctx) {
      ctx.clearRect(0, 0, canvas.width, canvas.height); // Clear canvas
      ctx.font = `${canvas.width * 0.05}px Arial`; // Adjust font size again
      ctx.fillText(username, canvas.width / 4, canvas.height / 2);
      ctx.fillText(opponentName, (3 * canvas.width) / 4, canvas.height / 2);
    }
  });
}

function displayMessage(text: string) {
  const messagesDiv = document.getElementById("messages");
  if (messagesDiv) {
    const messageElement = document.createElement("div");
    messageElement.textContent = text;
    messagesDiv.appendChild(messageElement);
    messagesDiv.scrollTop = messagesDiv.scrollHeight;
  }
}

function displayColoredMessage(text: string, color: string) {
  const messagesDiv = document.getElementById("messages");
  if (messagesDiv) {
    const messageElement = document.createElement("div");
    messageElement.textContent = text;
    messageElement.style.color = color;
    messagesDiv.appendChild(messageElement);
    messagesDiv.scrollTop = messagesDiv.scrollHeight;
  }
}

socket.addEventListener("open", () => {
  console.log("Connected to the WebSocket server.");
});

socket.addEventListener("message", (event) => {
  console.log(event.data);
  const data = JSON.parse(event.data);
  handleServerResponse(data);
});

socket.addEventListener("close", () => {
  console.log("Disconnected from the WebSocket server.");
});

socket.addEventListener("error", (error) => {
  console.error("WebSocket error:", error);
});

export function chat(message: string) {
  let messageType: MessageType = {
    type: "Text",
    data: message,
  };

  sendMessage(messageType);
}

export function join(username: string) {
  let joinRequest: MessageType = {
    type: "ConnectReq",
    data: username,
  };

  sendMessage(joinRequest);
}

export function startBattle() {
  let beginGame: MessageType = {
    type: "BeginGame",
  };

  sendMessage(beginGame);
}

function sendMessage(msg: MessageType) {
  let messageString: string = JSON.stringify(msg);
  socket.send(messageString);
}

(window as any).chat = chat;
(window as any).join = join;
(window as any).startBattle = startBattle;