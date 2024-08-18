use super::{
    service::ServerResponse,
    user::{User, UserStatus},
};
use crate::game::battle::Battle;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Default)]
pub struct State<'a> {
    users: HashMap<Uuid, User>,
    battles: HashMap<Uuid, Battle<'a>>,
}

impl<'a> State<'a> {
    pub fn get_name(&self, id: Uuid) -> Option<&String> {
        self.users[&id].name()
    }

    pub fn connect(&mut self, id: Uuid, user: User) {
        self.users.insert(id, user);
    }

    pub fn disconnect(&mut self, id: Uuid) {
        if self.users.contains_key(&id) {
            self.users.remove(&id);
        }
    }

    pub fn set_name(&mut self, id: Uuid, name: String) {
        if let Some(user) = self.users.get_mut(&id) {
            user.set_name(name)
        }
    }

    pub async fn broadcast(&mut self, msg: ServerResponse) -> ServerResult<()> {
        self.broadcast_to_all_but(msg, &[]).await
    }

    pub async fn broadcast_to_all_but(
        &mut self,
        msg: ServerResponse,
        exclude: &[Uuid],
    ) -> ServerResult<()> {
        for (_, user) in self
            .users
            .iter_mut()
            .filter(|(id, _)| !exclude.contains(id))
        {
            user.message(&msg)?.await?
        }
        Ok(())
    }

    pub fn new_random(&'a mut self) -> ServerResult<Uuid> {
        let mut rng = rand::thread_rng();
        let mut users = self.available_users();

        if users.len() < 2 {
            return Err(ServerError::NotEnoughInLobbyToStartError);
        }

        users.shuffle(&mut rng);
        let user_a = users.pop().unwrap();
        let user_b = users.pop().unwrap();

        self.new_battle(user_a, user_b)
    }

    pub fn new_battle(&'a mut self, user_a: Uuid, user_b: Uuid) -> ServerResult<Uuid> {
        let mut user_a = &self.users[&user_a];
        let mut user_b = &self.users[&user_b];

        if user_a.status() != &UserStatus::Lobby || user_b.status() != &UserStatus::Lobby {
            return Err(ServerError::AttemptedStartWhenNotInLobbyError);
        }

        let battle_id = Uuid::new_v4();
        let new_battle = Battle::start_battle(&mut user_a, &mut user_b);

        self.battles.insert(battle_id, new_battle);

        Ok(battle_id)
    }

    pub fn available_users(&self) -> Vec<Uuid> {
        self.users
            .iter()
            .filter(|(_, user)| user.status() == &UserStatus::Lobby)
            .map(|(id, _)| *id)
            .collect()
    }
}

// Might not need this, potentially will use mpsc and
// store the state on just a single thread? Not fully
// sure how it will work yet or if that can work :)

// pub type ServerState<'a> = Arc<Mutex<State<'a>>>;

#[derive(thiserror::Error, Debug)]
pub enum ServerError {
    #[error("Not enough users in lobby to start a battle")]
    NotEnoughInLobbyToStartError,
    #[error("Attempted to start a battle where at least one user is not in the lobby")]
    AttemptedStartWhenNotInLobbyError,
    #[error("Serde json Parse Error: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("No websocket attached to user")]
    SocketDisconnectedError,
    #[error("Tungstenite socket send error")]
    TungstentiteError(#[from] hyper_tungstenite::tungstenite::Error),
}

pub type ServerResult<T> = std::result::Result<T, ServerError>;
