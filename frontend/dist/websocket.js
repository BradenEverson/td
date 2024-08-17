export const socket = new WebSocket("/");
function handleServerResponse(response) {
    switch (response.message.type) {
        case "Chat":
            console.log(`Chat message from ${response.message.user}: ${response.message.message}`);
            break;
        case "GameStart":
            console.log(`Game started with ID: ${response.message.gameId}`);
            break;
        default:
            console.error("Unknown message type:", response.message);
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
function sendMessage(msg) {
    let messageString = JSON.stringify(msg);
    socket.send(messageString);
}
window.chat = chat;
window.join = join;
