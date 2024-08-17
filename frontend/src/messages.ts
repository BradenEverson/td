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

interface UserJoin {
  UserJoin: string;
}

export type ServerResponseType = GameStart | Chat | UserJoin;

export interface ServerResponse {
  message: ServerResponseType;
}
