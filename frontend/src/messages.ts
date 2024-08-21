export type Uuid = string;

export type MessageType =
  | { type: "ConnectReq"; data: string }
  | { type: "Text"; data: string }
  | { type: "Disconnect" }
  | { type: "BeginGame" };

interface Chat {
  Chat: [string, string];
}

interface GameStart {
  GameStart: Uuid;
}

interface UserJoin {
  UserJoin: string;
}

interface UserLeave {
  UserLeave: string;
}

interface StartGame {
  StartGame: [Uuid, Uuid];
}

export type ServerResponseType =
  | GameStart
  | Chat
  | UserJoin
  | UserLeave
  | StartGame;

export interface ServerResponse {
  message: ServerResponseType;
}
