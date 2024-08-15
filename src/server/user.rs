use uuid::Uuid;

use crate::game::entity::SpawnableEntity;

use super::service::WebSocketWriteStream;

#[derive(Default)]
pub struct User {
    id: Uuid,
    name: Option<String>,
    status: UserStatus,
    spawn_hand: [Option<SpawnableEntity>; 5],
    socket: Option<WebSocketWriteStream>,
}

impl User {
    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }
    pub fn set_socket(&mut self, socket: WebSocketWriteStream) {
        self.socket = Some(socket)
    }
    pub fn id(&self) -> &Uuid {
        &self.id
    }
    pub fn set_id(&mut self, id: Uuid) {
        self.id = id
    }
    pub fn status(&self) -> &UserStatus {
        &self.status
    }

    pub fn enter_game(&mut self, battle: Uuid) {
        self.status = UserStatus::InGame(battle)
    }

    pub fn leave_game(&mut self) {
        self.status = UserStatus::Lobby
    }
}

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub enum UserStatus {
    #[default]
    Lobby,
    InGame(Uuid),
}
