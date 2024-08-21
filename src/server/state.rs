use super::{
    service::ServerResponse,
    user::{User, UserStatus},
};
use crate::game::battle::Battle;
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Default)]
pub struct State {
    users: HashMap<Uuid, User>,
    battles: HashMap<Uuid, Battle>,
}

impl State {
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

    pub async fn broadcast_to(&mut self, msg: ServerResponse, to: &[Uuid]) -> ServerResult<()> {
        for (_, user) in self.users.iter_mut().filter(|(id, _)| to.contains(id)) {
            user.message(&msg)?.await?
        }
        Ok(())
    }

    pub fn new_random(&mut self, id: Uuid) -> ServerResult<(Uuid, Uuid)> {
        let mut rng = rand::thread_rng();
        let users: Vec<Uuid> = self.available_users(id);

        if users.len() < 1 {
            return Err(ServerError::NotEnoughInLobbyToStartError);
        }

        let oponent = users[rng.gen_range(0..users.len())];

        self.new_battle(id, oponent)
    }

    pub fn new_battle(&mut self, user_a_id: Uuid, user_b_id: Uuid) -> ServerResult<(Uuid, Uuid)> {
        {
            let user_a = &self.users[&user_a_id];
            let user_b = &self.users[&user_b_id];

            if user_a.status() != &UserStatus::Lobby || user_b.status() != &UserStatus::Lobby {
                return Err(ServerError::AttemptedStartWhenNotInLobbyError);
            }
        }

        let battle_id = Uuid::new_v4();
        let new_battle = Battle::start_battle(user_a_id, user_b_id);

        self.battles.insert(battle_id, new_battle);

        {
            self.users
                .get_mut(&user_a_id)
                .unwrap()
                .enter_game(battle_id);
            self.users
                .get_mut(&user_b_id)
                .unwrap()
                .enter_game(battle_id);
        }

        Ok((battle_id, user_b_id))
    }

    pub fn available_users(&self, exclude: Uuid) -> Vec<Uuid> {
        self.users
            .iter()
            .filter(|(id, user)| user.status() == &UserStatus::Lobby && *id != &exclude)
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
