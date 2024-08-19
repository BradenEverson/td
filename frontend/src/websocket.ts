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
  } else {
    console.log(response.message);
  }
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
    type: "BeginGame"
  };

  sendMessage(beginGame);
}

function sendMessage(msg: MessageType) {
  let messageString: string = JSON.stringify(msg);
  socket.send(messageString);
}

(window as any).chat = chat;
(window as any).join = join;
