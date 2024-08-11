use uuid::Uuid;

use crate::game::entity::SpawnableEntity;

#[derive(Default, Clone, PartialEq, Eq)]
pub struct User {
    name: String,
    id: Uuid,
    status: UserStatus,
    spawn_hand: [Option<SpawnableEntity>; 5],
}

impl User {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn id(&self) -> &Uuid {
        &self.id
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
