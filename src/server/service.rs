use futures::{executor::block_on, stream::SplitSink};
use futures_util::{Future, StreamExt};
use http_body_util::Full;
use hyper::{
    body::{self, Bytes},
    service::Service,
    upgrade::Upgraded,
};
use hyper::{Request, Response};
use hyper_tungstenite::HyperWebsocket;
use serde::{Deserialize, Serialize};
use std::{ops::Deref, pin::Pin};
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::{
    tungstenite::{Message, WebSocket},
    WebSocketStream,
};
use uuid::Uuid;

pub struct ServerService {
    pub sender: UnboundedSender<ServerMessage>,
}

pub type TokioMpscError = tokio::sync::mpsc::error::SendError<ServerMessage>;
pub type WebSocketReadStream =
    SplitSink<WebSocketStream<hyper_util::rt::tokio::TokioIo<Upgraded>>, Message>;

impl ServerService {
    pub fn new(tx: UnboundedSender<ServerMessage>) -> Self {
        Self { sender: tx }
    }
    pub fn send_msg(&mut self, msg: ServerMessage) -> Result<(), TokioMpscError> {
        self.sender.send(msg)
    }
    pub fn chat(&mut self, from: Uuid, msg: &str) -> Result<(), TokioMpscError> {
        let message = ServerMessage::text(from, msg);
        self.sender.send(message)
    }
    pub fn websocket(&mut self, socket: WebSocketReadStream) -> Result<(), TokioMpscError> {
        let message = ServerMessage::new(None, MessageType::ConnectWs(socket));
        self.sender.send(message)
    }
}

impl Service<Request<body::Incoming>> for ServerService {
    type Response = Response<Full<Bytes>>;
    type Error = hyper::http::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, mut req: Request<body::Incoming>) -> Self::Future {
        let tx = self.sender.clone();
        if hyper_tungstenite::is_upgrade_request(&req) {
            // Upgrade to WebSocket
            let (_, websocket) =
                hyper_tungstenite::upgrade(&mut req, None).expect("Error upgrading to WebSocket");
            let (writer, mut reader) = block_on(async { websocket.await.unwrap().split() });

            let user_id = Uuid::new_v4();

            tx.send(ServerMessage::new(
                Some(user_id),
                MessageType::ConnectWs(writer),
            ))
            .expect("Failed to send websocket write stream up channel");

            tokio::spawn(async move {
                while let Some(Ok(msg)) = reader.next().await {
                    // TODO - Respond to websocket messages accordingly
                    match msg {
                        Message::Text(txt) => {
                            tx.send(ServerMessage::text(user_id, &txt))?;
                        }
                        _ => {}
                    }
                }
                Ok::<(), TokioMpscError>(())
            });

            todo!();
        } else {
            // Traditional HTTP
            todo!();
        }
    }
}

#[derive(Debug)]
pub struct ServerMessage {
    pub from: Option<Uuid>,
    pub msg: MessageType,
}

impl ServerMessage {
    fn new(from: Option<Uuid>, msg: MessageType) -> Self {
        Self { from, msg }
    }

    fn text(from: Uuid, msg: &str) -> Self {
        Self::new(Some(from), MessageType::Text(msg.to_string()))
    }
}

#[derive(Debug)]
pub enum MessageType {
    ConnectReq(String),
    Text(String),
    ConnectWs(WebSocketReadStream),
    Disconnect,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerResponse {
    from: Option<String>,
    message: ResponseType,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ResponseType {
    Chat(String),
    GameStart(Uuid),
}
