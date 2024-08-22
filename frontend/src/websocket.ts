import { MessageType, ServerResponse, ServerResponseType, Unit } from "./messages";

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
    const userName = response.message.StartGame[0];
    const opponentName = response.message.StartGame[1];
    switchToGameView(userName, opponentName);
  } else if ("DrawnHand" in response.message) {
    const drawnHand: Array<Unit> = response.message.DrawnHand;

    const canvas = document.getElementById("game-canvas") as HTMLCanvasElement;
    const ctx = canvas?.getContext("2d");

    let userMoney = 50;

    if (ctx) {
      const buttonWidth = canvas.width / drawnHand.length;
      const buttonHeight = canvas.height * 0.2;

      const cooldowns: Array<number> = drawnHand.map(unit => unit.power * unit.speed * 500);
      const cooldownStartTimes: Array<number> = new Array(drawnHand.length).fill(Date.now());

      function drawMoney() {
        if (ctx) {
          ctx.clearRect(canvas.width - 200, 0, 200, 50);
          ctx.font = "30px Arial";
          ctx.fillStyle = "#ffffff";
          ctx.textAlign = "right";
          ctx.fillText(`Money: ${userMoney}`, canvas.width - 10, 40);
        }
      }

      function drawButtons() {
        if (ctx) {
          ctx.clearRect(0, canvas.height - buttonHeight, canvas.width, buttonHeight);

          const now = Date.now();

          drawnHand.forEach((unit, index) => {
            const x = index * buttonWidth;
            const y = canvas.height - buttonHeight;

            const cooldownDuration = cooldowns[index];
            const elapsed = now - cooldownStartTimes[index];
            const remainingCooldown = Math.max(cooldownDuration - elapsed, 0);
            const cooldownPercentage = remainingCooldown / cooldownDuration;

            ctx.fillStyle = remainingCooldown > 0 ? "#777777" : "#a37b48";
            ctx.fillRect(x, y, buttonWidth, buttonHeight);

            ctx.strokeStyle = "#654321";
            ctx.lineWidth = 5;
            ctx.strokeRect(x, y, buttonWidth, buttonHeight);

            const emojiSize = buttonHeight * 0.6;
            ctx.font = `${emojiSize}px Arial`;
            ctx.textAlign = "center";
            ctx.fillStyle = "#ffffff";
            ctx.fillText(unit.emoji, x + buttonWidth / 2, y + emojiSize);

            if (remainingCooldown > 0) {
              ctx.fillStyle = "rgba(0, 0, 0, 0.5)";
              ctx.fillRect(x, y, buttonWidth, buttonHeight * cooldownPercentage);
            }

            const priceFontSize = buttonHeight * 0.2;
            ctx.font = `bold ${priceFontSize}px Arial`;
            ctx.fillStyle = "#ffe400";
            const priceText = `\$${unit.cost}`;
            const textMetrics = ctx.measureText(priceText);
            const priceX = x + buttonWidth / 2 - textMetrics.width / 2;
            const priceY = y + buttonHeight - priceFontSize * 0.5;
            ctx.fillText(priceText, priceX + textMetrics.width / 2, priceY);
          });

          drawMoney();
        }

      }

      drawButtons();

      canvas.addEventListener("click", (event) => {
        const rect = canvas.getBoundingClientRect();
        const clickX = event.clientX - rect.left;
        const clickY = event.clientY - rect.top;

        drawnHand.forEach((unit, index) => {
          const x = index * buttonWidth;
          const y = canvas.height - buttonHeight;

          const cooldownDuration = cooldowns[index];
          const now = Date.now();
          const elapsed = now - cooldownStartTimes[index];
          const remainingCooldown = Math.max(cooldownDuration - elapsed, 0);

          if (
            clickX > x &&
            clickX < x + buttonWidth &&
            clickY > y &&
            clickY < y + buttonHeight &&
            remainingCooldown === 0 &&
            userMoney >= unit.cost
          ) {
            console.log(`Spawning unit: ${unit.name}`);

            userMoney -= unit.cost;
            cooldownStartTimes[index] = Date.now();
            drawButtons();
          }
        });
      });

      setInterval(() => {
        userMoney += 1;
        drawButtons();
      }, 100);
    }
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

    ctx.fillText(username, canvas.width / 4, canvas.height / 2);
    ctx.fillText(opponentName, (3 * canvas.width) / 4, canvas.height / 2);
  }

  window.addEventListener("resize", () => {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    if (ctx) {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      ctx.font = `${canvas.width * 0.05}px Arial`;
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