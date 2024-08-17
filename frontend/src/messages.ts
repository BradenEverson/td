export type Uuid = string;

export type MessageType =
  | { type: "ConnectReq"; data: string }
  | { type: "Text"; data: string }
  | { type: "Disconnect" };

export type ServerResponseType =
  | { type: "Chat"; user: string; message: string }
  | { type: "GameStart"; gameId: Uuid };

export interface ServerResponse {
  message: ServerResponseType;
}
