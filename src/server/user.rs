use futures::{sink::Send, SinkExt};
use tokio_tungstenite::tungstenite::Message;
use uuid::Uuid;

use crate::game::entity::Unit;

use super::{
    service::{ServerResponse, WebSocketWriteStream},
    state::{ServerError, ServerResult, GAME_HAND_SIZE},
};

#[derive(Default)]
pub struct User<'a> {
    id: Uuid,
    name: Option<String>,
    status: UserStatus,
    spawn_hand: Option<[Unit<'a>; GAME_HAND_SIZE]>,
    socket: Option<WebSocketWriteStream>,
}

impl<'a> User<'a> {
    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }
    pub fn set_socket(&mut self, socket: WebSocketWriteStream) {
        self.socket = Some(socket)
    }
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name)
    }

    pub fn set_id(&mut self, id: Uuid) {
        self.id = id
    }

    pub fn get_hand(&self) -> Option<[Unit<'a>; GAME_HAND_SIZE]> {
        self.spawn_hand
    }

    pub fn status(&self) -> &UserStatus {
        &self.status
    }

    pub fn enter_game(&mut self, battle: Uuid, hand: [Unit<'a>; 5]) {
        self.status = UserStatus::InGame(battle);
        self.set_hand(hand)
    }

    pub fn leave_game(&mut self) {
        self.status = UserStatus::Lobby
    }

    pub fn set_hand(&mut self, hand: [Unit<'a>; 5]) {
        self.spawn_hand = Some(hand.to_owned());
    }

    pub fn message(
        &mut self,
        message: &ServerResponse,
    ) -> ServerResult<
        Send<
            futures::prelude::stream::SplitSink<
                tokio_tungstenite::WebSocketStream<
                    hyper_util::rt::TokioIo<hyper::upgrade::Upgraded>,
                >,
                Message,
            >,
            Message,
        >,
    > {
        if let Some(socket) = &mut self.socket {
            let msg = serde_json::to_string(&message)?;
            Ok(socket.send(Message::text(msg)))
        } else {
            Err(ServerError::SocketDisconnectedError)
        }
    }
}

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub enum UserStatus {
    #[default]
    Lobby,
    InGame(Uuid),
}
