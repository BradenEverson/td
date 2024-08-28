import {
  MessageType,
  ServerResponse,
  Unit,
} from "./messages";

export const socket = new WebSocket("/");

type UnitMetadata = {
  unit: Unit;
  position: [number, number];
  target: [number, number];

  attackCooldown: number;

  t: number
};

let gameDone: boolean = false;

let playerUnits: Array<UnitMetadata> = [];
let enemyUnits: Array<UnitMetadata> = [];
let drawnHand: Array<Unit> | null = null;
let drawnHandSetup = false;

let cooldownStartTimes: Array<number>;
let cooldowns: Array<number>;

let userMoney: number = 50;

let opponentTowerX: number;
let opponentTowerY: number;

let userTowerX: number;
let userTowerY: number;

let userTowerHealth: number = 15000;
let enemyTowerHealth: number = 15000;

function handleServerResponse(response: ServerResponse) {
  if ("Chat" in response.message) {
    let message: string =
      response.message.Chat[0] + ": " + response.message.Chat[1];
    if (response.message.Chat[0] == "Server") {
      displayColoredMessage(message, "#a32791");
    } else {
      displayMessage(message);
    }
  } else if ("UserJoin" in response.message) {
    let message: string = response.message.UserJoin + " has joined the server";
    displayColoredMessage(message, "#80a4bf");
  } else if ("NewTowerHealth" in response.message) {
    let [user, health] = response.message.NewTowerHealth;
    if (user) {
      userTowerHealth = health;
    } else {
      enemyTowerHealth = health;
    }
  } else if ("StartGame" in response.message) {
    const userName = response.message.StartGame[0];
    const opponentName = response.message.StartGame[1];
    switchToGameView(userName, opponentName);
  } else if ("DrawnHand" in response.message) {
    drawnHand = response.message.DrawnHand;

    setInterval(() => {
      userMoney += 1;
    }, 100);
  } else if ("UnitSpawned" in response.message) {
    const canvas = document.getElementById("game-canvas") as HTMLCanvasElement;

    const towerSize = canvas.width * 0.1;
    const towerPadding = canvas.width * 0.05;
    userTowerX = towerPadding + towerSize / 2;
    userTowerY = canvas.height * 0.7;
    opponentTowerX = canvas.width - towerPadding - towerSize / 2;
    opponentTowerY = canvas.height * 0.7;

    let [isOurs, unit] = response.message.UnitSpawned;

    if (isOurs) {
      let unit_metadata: UnitMetadata = {
        unit: unit,
        position: [userTowerX, userTowerY],
        target: [opponentTowerX, opponentTowerY],
        attackCooldown: 0,
        t: 0
      };

      playerUnits.push(unit_metadata);
    } else {
      let unit_metadata: UnitMetadata = {
        unit: unit,
        position: [opponentTowerX, opponentTowerY],
        target: [userTowerX, userTowerY],
        attackCooldown: 0,
        t: 0
      };

      enemyUnits.push(unit_metadata);
    }
  } else if ("WinByDisconnect" in response.message && !gameDone) {
    alert("Opponent has left, you win!");
    gameDone = true;
  } else if ("Win" in response.message) {
    alert("You have defeated your opponent! Final health of your tower was: " + userTowerHealth);
    gameDone = true;
  } else if ("Lose" in response.message) {
    alert("Your enemy has destroyed your tower, you lose.");
    gameDone = true;
  } else {
    console.log(response.message);
  }

  if (gameDone) {
    window.location.reload();
  }
}

function dist(pointA: [number, number], pointB: [number, number]): number {
  return Math.sqrt(Math.pow((pointB[0] - pointA[0]), 2) + Math.pow((pointB[1] - pointA[1]), 2));
}

function findClosestEnemy(unit: UnitMetadata, enemies: Array<UnitMetadata>): UnitMetadata | null {
  let closestEnemy: UnitMetadata | null = null;
  let minDistance = Infinity;

  for (let i = 0; i < enemies.length; i++) {
    let enemy = enemies[i];
    let distance = dist(unit.position, enemy.position);
    if (distance < minDistance) {
      minDistance = distance;
      closestEnemy = enemy;
    }
  }

  return closestEnemy;
}

function updateAllUnits() {
  for (let i = 0; i < playerUnits.length; i++) {
    let playerUnit = playerUnits[i];

    let closestEnemy = findClosestEnemy(playerUnit, enemyUnits);
    if (closestEnemy && dist(playerUnit.position, closestEnemy.position) <= (playerUnit.unit.size + closestEnemy.unit.size) * 22.5) {
      playerUnit.attackCooldown += playerUnit.unit.speed / 10;
      if (playerUnit.attackCooldown >= 100) {
        closestEnemy.unit.health -= playerUnit.unit.power;
        playerUnit.attackCooldown = 0;

        if (closestEnemy.unit.health <= 0) {
          enemyUnits.splice(enemyUnits.indexOf(closestEnemy), 1);
          userMoney += closestEnemy.unit.cost / 10;
        }
      }
    } else {
      let distance = dist(playerUnit.position, [opponentTowerX, opponentTowerY]);
      if ((distance - (playerUnit.unit.size * 22.5)) < 3) {
        playerUnit.attackCooldown += playerUnit.unit.speed / 10;
        if (playerUnit.attackCooldown >= 100) {
          damagePing(playerUnit.unit.power);
          playerUnit.attackCooldown = 0;
        }
      } else {
        playerUnit.position[0] += playerUnit.unit.speed / 10;
      }
    }
    playerUnit.t += 1 / (playerUnit.unit.speed * 10);
  }

  for (let i = 0; i < enemyUnits.length; i++) {
    let enemyUnit = enemyUnits[i];

    let closestPlayerUnit = findClosestEnemy(enemyUnit, playerUnits);
    if (closestPlayerUnit && dist(enemyUnit.position, closestPlayerUnit.position) <= (enemyUnit.unit.size + closestPlayerUnit.unit.size) * 22.5) {
      enemyUnit.attackCooldown += enemyUnit.unit.speed / 10;
      if (enemyUnit.attackCooldown >= 100) {
        closestPlayerUnit.unit.health -= enemyUnit.unit.power;
        enemyUnit.attackCooldown = 0;

        if (closestPlayerUnit.unit.health <= 0) {
          playerUnits.splice(playerUnits.indexOf(closestPlayerUnit), 1);
        }
      }
    } else {
      let distance = dist(enemyUnit.position, [userTowerX, userTowerY]);
      if ((distance - enemyUnit.unit.size) < 3) {
        enemyUnit.attackCooldown += enemyUnit.unit.speed / 10;
        if (enemyUnit.attackCooldown >= 100) {
          enemyUnit.attackCooldown = 0;
        }
      } else {
        enemyUnit.position[0] -= enemyUnit.unit.speed / 10;
      }
    }
    enemyUnit.t += 1 / (enemyUnit.unit.speed * 10);
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
    let buttonWidth: number;
    let buttonHeight: number;

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

        // Draw the user tower's health bar
        {
          const healthBarWidth = towerSize;
          const healthBarHeight = canvas.height * 0.01;
          const healthBarX = userTowerX - healthBarWidth / 2;
          const healthBarY = userTowerY - (towerSize * 0.75) - healthBarHeight;

          ctx.fillStyle = "red";
          ctx.fillRect(healthBarX, healthBarY, healthBarWidth, healthBarHeight);

          ctx.fillStyle = "green";
          ctx.fillRect(
            healthBarX,
            healthBarY,
            healthBarWidth * (enemyTowerHealth / 15000),
            healthBarHeight
          );
        }

        {
          const healthBarWidth = towerSize;
          const healthBarHeight = canvas.height * 0.01;
          const healthBarX = opponentTowerX - healthBarWidth / 2;
          const healthBarY = opponentTowerY - (towerSize * 0.75) - healthBarHeight;

          ctx.fillStyle = "red";
          ctx.fillRect(healthBarX, healthBarY, healthBarWidth, healthBarHeight);

          ctx.fillStyle = "green";
          ctx.fillRect(
            healthBarX,
            healthBarY,
            healthBarWidth * (userTowerHealth / 15000),
            healthBarHeight
          );
        }


        // Draw The Card Buttons:
        if (drawnHand) {
          if (!drawnHandSetup) {
            drawnHandSetup = true;

            cooldowns = drawnHand.map(
              (unit) => unit.power * (1 / unit.speed) * 500,
            );

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
              ctx.fillRect(
                x,
                y,
                buttonWidth,
                buttonHeight * cooldownPercentage,
              );
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
          ctx.save();
          ctx.translate(unit.position[0], unit.position[1] + (2 * Math.sin(unit.t / 2)));
          ctx.rotate(-1 * unit.attackCooldown / 300);
          ctx.font = `${45 * unit.unit.size}px Arial`;
          ctx.fillText(unit.unit.emoji, 0, 0);
          ctx.restore();
        }
        for (let i = 0; i < enemyUnits.length; i++) {
          let unit = enemyUnits[i];
          ctx.save();
          ctx.translate(unit.position[0], unit.position[1] + (2 * Math.sin(unit.t / 2)));
          ctx.rotate(unit.attackCooldown / 300);
          ctx.font = `${45 * unit.unit.size}px Arial`;
          ctx.fillText(unit.unit.emoji, 0, 0);
          ctx.restore();
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

          if (
            clickX > x &&
            clickX < x + buttonWidth &&
            clickY > y &&
            clickY < y + buttonHeight &&
            remainingCooldown === 0 &&
            userMoney >= unit.cost
          ) {
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

export function sendUnit(unitId: string) {
  let sendUnit: MessageType = {
    type: "SpawnUnit",
    data: unitId,
  };

  sendMessage(sendUnit);
}

export function startBattle() {
  let beginGame: MessageType = {
    type: "BeginGame",
  };

  sendMessage(beginGame);
}

function damagePing(dmg: number) {
  let msg: MessageType = {
    type: "DmgPing",
    data: dmg.toString()
  };

  sendMessage(msg);
}

function sendMessage(msg: MessageType) {
  let messageString: string = JSON.stringify(msg);
  socket.send(messageString);
}

(window as any).chat = chat;
(window as any).join = join;
(window as any).startBattle = startBattle;
