export type Uuid = string;

export type MessageType =
  | { type: "ConnectReq"; data: string }
  | { type: "Text"; data: string }
  | { type: "Disconnect" }
  | { type: "BeginGame" }
  | { type: "SpawnUnit"; data: string };

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

interface DrawnHand {
  DrawnHand: Array<Unit>;
}

export type Unit = {
  name: string;
  emoji: string;
  cost: number;
  health: number;
  power: number;
  size: number;
  speed: number;
  attack_type: Attack;
};

export type Attack = "Area" | "Single";

export type ServerResponseType =
  | GameStart
  | Chat
  | UserJoin
  | UserLeave
  | StartGame
  | DrawnHand;

export interface ServerResponse {
  message: ServerResponseType;
}
