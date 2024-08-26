export type Uuid = string;

export type MessageType =
  | { type: "ConnectReq"; data: string }
  | { type: "Text"; data: string }
  | { type: "Disconnect" }
  | { type: "BeginGame" }
  | { type: "SpawnUnit"; data: string }
  | { type: "DmgPing"; data: string };

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

interface UnitSpawned {
  UnitSpawned: [boolean, Unit];
}

interface NewTowerHealth {
  NewTowerHealth: [boolean, number];
}

interface Win { };
interface Lose { };
interface WinByDisconnect { };

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
  | DrawnHand
  | UnitSpawned
  | NewTowerHealth
  | Win
  | WinByDisconnect
  | Lose;

export interface ServerResponse {
  message: ServerResponseType;
}
