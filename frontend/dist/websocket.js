export const socket = new WebSocket("/");
let playerUnits = [];
let enemyUnits = [];
let drawnHand = null;
let drawnHandSetup = false;
let cooldownStartTimes;
let cooldowns;
let userMoney = 50;
let opponentTowerX;
let opponentTowerY;
let userTowerX;
let userTowerY;
function handleServerResponse(response) {
    if ("Chat" in response.message) {
        let message = response.message.Chat[0] + ": " + response.message.Chat[1];
        displayMessage(message);
    }
    else if ("UserJoin" in response.message) {
        let message = response.message.UserJoin + " has joined the server";
        displayColoredMessage(message, "#80a4bf");
    }
    else if ("UserLeave" in response.message) {
        let message = response.message.UserLeave + " has disconnected :(";
        displayColoredMessage(message, "#f07269");
    }
    else if ("StartGame" in response.message) {
        const userName = response.message.StartGame[0];
        const opponentName = response.message.StartGame[1];
        switchToGameView(userName, opponentName);
    }
    else if ("DrawnHand" in response.message) {
        drawnHand = response.message.DrawnHand;
        setInterval(() => {
            userMoney += 1;
        }, 100);
    }
    else if ("UnitSpawned" in response.message) {
        const canvas = document.getElementById("game-canvas");
        const towerSize = canvas.width * 0.1;
        const towerPadding = canvas.width * 0.05;
        userTowerX = towerPadding + towerSize / 2;
        userTowerY = canvas.height * 0.7;
        opponentTowerX = canvas.width - towerPadding - towerSize / 2;
        opponentTowerY = canvas.height * 0.7;
        let [isOurs, unit] = response.message.UnitSpawned;
        if (isOurs) {
            let unit_metadata = {
                unit: unit,
                position: [userTowerX, userTowerY],
                target: [opponentTowerX, opponentTowerY],
                attackCooldown: 0,
                t: 0
            };
            playerUnits.push(unit_metadata);
        }
        else {
            let unit_metadata = {
                unit: unit,
                position: [opponentTowerX, opponentTowerY],
                target: [userTowerX, userTowerY],
                attackCooldown: 0,
                t: 0
            };
            enemyUnits.push(unit_metadata);
        }
    } /*else if ("WinByDisconnect" in response.message) {
      alert("Opponent has left, you win!");
      window.location.reload();
    } */
    else {
        console.log(response.message);
    }
}
function dist(pointA, pointB) {
    return Math.sqrt(Math.pow((pointB[0] - pointA[0]), 2) + Math.pow((pointB[1] - pointA[1]), 2));
}
function updateAllUnits() {
    for (let i = 0; i < playerUnits.length; i++) {
        let unit = playerUnits[i];
        let distance = dist(unit.position, [opponentTowerX, opponentTowerY]);
        //console.log(unit.unit.name + " distance from tower: " + distance);
        if ((distance - unit.unit.size) < 3) {
            unit.attackCooldown += unit.unit.speed / 10;
            if (unit.attackCooldown >= 100) {
                //Send tower attack to server
                unit.attackCooldown = 0;
            }
        }
        else {
            unit.position[0] += unit.unit.speed / 10;
        }
        unit.t += 1 / (unit.unit.speed * 10);
    }
    for (let i = 0; i < enemyUnits.length; i++) {
        let unit = enemyUnits[i];
        let distance = dist(unit.position, [userTowerX, userTowerY]);
        //console.log(unit.unit.name + " distance from tower: " + distance);
        if ((distance - unit.unit.size) < 3) {
            unit.attackCooldown += unit.unit.speed / 10;
            if (unit.attackCooldown >= 100) {
                //Send tower attack to server
                unit.attackCooldown = 0;
            }
        }
        else {
            unit.position[0] -= unit.unit.speed / 10;
        }
        unit.t += 1 / (unit.unit.speed * 10);
    }
}
function switchToGameView(username, opponentName) {
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
        let buttonWidth;
        let buttonHeight;
        const towerSize = canvas.width * 0.1;
        const towerPadding = canvas.width * 0.05;
        function drawBattlefield() {
            if (ctx) {
                ctx.clearRect(0, 0, canvas.width, canvas.height);
                ctx.fillStyle = "#87CEEB";
                ctx.fillRect(0, 0, canvas.width, canvas.height * 0.7);
                ctx.fillStyle = "#228B22";
                ctx.fillRect(0, canvas.height * 0.7, canvas.width, canvas.height * 0.3);
                const opponentTowerX = towerPadding + towerSize / 2;
                const opponentTowerY = canvas.height * 0.7;
                ctx.font = `${towerSize}px Arial`;
                ctx.textAlign = "center";
                const userTowerX = canvas.width - towerPadding - towerSize / 2;
                const userTowerY = canvas.height * 0.7;
                ctx.fillText("🏡", opponentTowerX, opponentTowerY);
                ctx.fillText("🏡", userTowerX, userTowerY);
                ctx.fillStyle = "#FFFFFF";
                ctx.font = `${canvas.width * 0.03}px Arial`;
                ctx.fillText(username, userTowerX, userTowerY - towerSize);
                ctx.fillText(opponentName, opponentTowerX, opponentTowerY - towerSize);
                // Draw The Card Buttons:
                if (drawnHand) {
                    if (!drawnHandSetup) {
                        drawnHandSetup = true;
                        cooldowns = drawnHand.map((unit) => unit.power * (1 / unit.speed) * 500);
                        cooldownStartTimes = new Array(drawnHand.length).fill(Date.now());
                    }
                    buttonWidth = canvas.width / drawnHand.length;
                    buttonHeight = canvas.height * 0.2;
                    const now = Date.now();
                    drawnHand.forEach((unit, index) => {
                        const x = index * buttonWidth;
                        const y = canvas.height - buttonHeight;
                        const cooldownDuration = cooldowns[index];
                        const elapsed = now - cooldownStartTimes[index];
                        const remainingCooldown = Math.max(cooldownDuration - elapsed, 0);
                        const cooldownPercentage = remainingCooldown / cooldownDuration;
                        ctx.fillStyle =
                            remainingCooldown > 0
                                ? "#777777"
                                : userMoney >= unit.cost
                                    ? "#a37b48"
                                    : "#91897e";
                        ctx.fillRect(x, y, buttonWidth, buttonHeight);
                        ctx.strokeStyle = "#654321";
                        ctx.lineWidth = 5;
                        ctx.strokeRect(x, y, buttonWidth, buttonHeight);
                        const emojiSize = buttonHeight * 0.6;
                        ctx.font =
                            userMoney < unit.cost && remainingCooldown <= 0
                                ? `italic ${emojiSize}px Arial`
                                : `${emojiSize}px Arial`;
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
                }
                updateAllUnits();
                for (let i = 0; i < playerUnits.length; i++) {
                    let unit = playerUnits[i];
                    ctx.font = `${45 * unit.unit.size}px Arial`;
                    ctx.fillText(unit.unit.emoji, unit.position[0], unit.position[1] + (2 * Math.sin(unit.t / 2)));
                }
                for (let i = 0; i < enemyUnits.length; i++) {
                    let unit = enemyUnits[i];
                    ctx.font = `${45 * unit.unit.size}px Arial`;
                    ctx.fillText(unit.unit.emoji, unit.position[0], unit.position[1] + (2 * Math.sin(unit.t / 2)));
                }
                ctx.clearRect(canvas.width - 200, 0, 200, 50);
                ctx.font = "30px Arial";
                ctx.textAlign = "right";
                ctx.fillStyle = "#87CEEB";
                ctx.fillRect(canvas.width - 200, 0, 200, 50);
                ctx.fillStyle = "#ffffff";
                ctx.fillText(`Money: ${userMoney}`, canvas.width - 10, 40);
            }
        }
        window.addEventListener("resize", () => {
            canvas.width = window.innerWidth;
            canvas.height = window.innerHeight;
        });
        canvas.addEventListener("click", (event) => {
            const rect = canvas.getBoundingClientRect();
            const clickX = event.clientX - rect.left;
            const clickY = event.clientY - rect.top;
            if (drawnHand) {
                drawnHand.forEach((unit, index) => {
                    const x = index * buttonWidth;
                    const y = canvas.height - buttonHeight;
                    const cooldownDuration = cooldowns[index];
                    const now = Date.now();
                    const elapsed = now - cooldownStartTimes[index];
                    const remainingCooldown = Math.max(cooldownDuration - elapsed, 0);
                    if (clickX > x &&
                        clickX < x + buttonWidth &&
                        clickY > y &&
                        clickY < y + buttonHeight &&
                        remainingCooldown === 0 &&
                        userMoney >= unit.cost) {
                        sendUnit(unit.name);
                        userMoney -= unit.cost;
                        cooldownStartTimes[index] = Date.now();
                    }
                });
            }
        });
        setInterval(drawBattlefield, 10);
    }
}
function displayMessage(text) {
    const messagesDiv = document.getElementById("messages");
    if (messagesDiv) {
        const messageElement = document.createElement("div");
        messageElement.textContent = text;
        messagesDiv.appendChild(messageElement);
        messagesDiv.scrollTop = messagesDiv.scrollHeight;
    }
}
function displayColoredMessage(text, color) {
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
export function chat(message) {
    let messageType = {
        type: "Text",
        data: message,
    };
    sendMessage(messageType);
}
export function join(username) {
    let joinRequest = {
        type: "ConnectReq",
        data: username,
    };
    sendMessage(joinRequest);
}
export function sendUnit(unitId) {
    let sendUnit = {
        type: "SpawnUnit",
        data: unitId,
    };
    sendMessage(sendUnit);
}
export function startBattle() {
    let beginGame = {
        type: "BeginGame",
    };
    sendMessage(beginGame);
}
function sendMessage(msg) {
    let messageString = JSON.stringify(msg);
    socket.send(messageString);
}
window.chat = chat;
window.join = join;
window.startBattle = startBattle;
