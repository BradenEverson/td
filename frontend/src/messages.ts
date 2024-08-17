export type Uuid = string;

export type MessageType =
  | { type: "ConnectReq"; data: string }
  | { type: "Text"; data: string }
  | { type: "Disconnect" };

interface Chat {
  Chat: [string, string];
}

interface GameStart {
  GameStart: Uuid;
}

export type ServerResponseType = GameStart | Chat;

export interface ServerResponse {
  message: ServerResponseType;
}
