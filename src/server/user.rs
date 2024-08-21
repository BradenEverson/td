use futures::{sink::Send, SinkExt};
use tokio_tungstenite::tungstenite::Message;
use uuid::Uuid;

use crate::game::entity::Unit;

use super::{
    service::{ServerResponse, WebSocketWriteStream},
    state::{ServerError, ServerResult},
};

#[derive(Default)]
pub struct User {
    id: Uuid,
    name: Option<String>,
    status: UserStatus,
    spawn_hand: [Option<Unit>; 5],
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

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name)
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
