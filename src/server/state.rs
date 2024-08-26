use super::{
    service::{ResponseType, ServerResponse},
    user::{User, UserStatus},
};
use crate::game::{
    battle::Battle,
    entity::{draw_hand, Unit},
};
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

pub const GAME_HAND_SIZE: usize = 5;

#[derive(Default)]
pub struct State<'a> {
    users: HashMap<Uuid, User<'a>>,
    battles: HashMap<Uuid, Battle>,
}

impl<'a> State<'a> {
    pub fn get_name(&self, id: Uuid) -> Option<&String> {
        self.users[&id].name()
    }

    pub fn connect(&mut self, id: Uuid, user: User<'a>) {
        self.users.insert(id, user);
    }

    pub fn disconnect(&mut self, id: Uuid) {
        if self.users.contains_key(&id) {
            self.users.remove(&id);
        }
    }

    pub fn damage(&mut self, to: Uuid, dmg: usize) -> Option<usize> {
        if let UserStatus::InGame(battle_id) = *self.users[&to].status() {
            let battle = self.battles.get_mut(&battle_id).unwrap();

            battle.damage_tick(to, dmg)
        } else {
            None
        }
    }

    pub fn get_opponent(&self, id: Uuid) -> Option<Uuid> {
        let user_state = *self.users[&id].status();
        if let UserStatus::InGame(battle_id) = user_state {
            let battle = self.battles[&battle_id];

            Some(battle.get_enemy(id))
        } else {
            None
        }
    }

    pub fn get_card_from_user(&self, id: Uuid, card: usize) -> ServerResult<Unit> {
        let user = &self.users[&id];

        if let Some(card_in_hand) = user.get_card(card) {
            Ok(card_in_hand.clone())
        } else {
            Err(ServerError::NoHandYetError)
        }
    }

    pub fn set_name(&mut self, id: Uuid, name: String) {
        if let Some(user) = self.users.get_mut(&id) {
            user.set_name(name)
        }
    }

    pub async fn broadcast(&mut self, msg: ServerResponse<'_>) -> ServerResult<()> {
        self.broadcast_to_all_but(msg, &[]).await
    }

    pub async fn broadcast_to_all_but(
        &mut self,
        msg: ServerResponse<'_>,
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

    pub async fn broadcast_to(&mut self, msg: ServerResponse<'_>, to: &[Uuid]) -> ServerResult<()> {
        for (_, user) in self.users.iter_mut().filter(|(id, _)| to.contains(id)) {
            user.message(&msg)?.await?
        }
        Ok(())
    }

    pub async fn broadcast_users_hand(&mut self, id: Uuid) -> ServerResult<()> {
        if !self.users.contains_key(&id) {
            return Err(ServerError::InvalidUserIdError);
        }

        let user = self.users.get_mut(&id).unwrap();
        let hand = user.get_hand();

        if hand.is_none() {
            return Err(ServerError::NoHandYetError);
        }

        let hand = hand.unwrap();
        let response = ServerResponse::new(ResponseType::DrawnHand(Box::new(hand)));

        user.message(&response)?.await?;

        Ok(())
    }

    pub fn new_random(&mut self, id: Uuid) -> ServerResult<(Uuid, Uuid)> {
        let mut rng = rand::thread_rng();
        let users: Vec<Uuid> = self.available_users(id);

        if users.is_empty() {
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

        let hand_a = draw_hand::<GAME_HAND_SIZE>().unwrap();
        let hand_b = draw_hand::<GAME_HAND_SIZE>().unwrap();

        {
            self.users
                .get_mut(&user_a_id)
                .unwrap()
                .enter_game(battle_id, hand_a);
            self.users
                .get_mut(&user_b_id)
                .unwrap()
                .enter_game(battle_id, hand_b);
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
    #[error("Requested hand when it did not exist")]
    NoHandYetError,
    #[error("User does not exist")]
    InvalidUserIdError,
}

pub type ServerResult<T> = std::result::Result<T, ServerError>;
