use futures_util::Future;
use http_body_util::Full;
use hyper::{
    body::{self, Bytes},
    service::Service,
};
use hyper::{Request, Response};
use hyper_tungstenite::HyperWebsocket;
use serde::{Deserialize, Serialize};
use std::{ops::Deref, pin::Pin};
use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;

pub struct ServerService {
    sender: UnboundedSender<ServerMessage>,
}

pub type TokioMpscResult<T> = Result<T, tokio::sync::mpsc::error::SendError<ServerMessage>>;

impl ServerService {
    pub fn new(tx: UnboundedSender<ServerMessage>) -> Self {
        Self { sender: tx }
    }
    pub fn send_msg(&mut self, msg: ServerMessage) -> TokioMpscResult<()> {
        self.sender.send(msg)
    }
    pub fn chat(&mut self, from: Uuid, msg: &str) -> TokioMpscResult<()> {
        let message = ServerMessage::text(from, msg);
        self.sender.send(message)
    }
}

impl Service<Request<body::Incoming>> for ServerService {
    type Response = Response<Full<Bytes>>;
    type Error = hyper::http::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, mut req: Request<body::Incoming>) -> Self::Future {
        if hyper_tungstenite::is_upgrade_request(&req) {
            // Upgrade to WebSocket
            let (response, websocket) =
                hyper_tungstenite::upgrade(&mut req, None).expect("Error upgrading to WebSocket");
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
    fn text(from: Uuid, msg: &str) -> Self {
        Self {
            from: Some(from),
            msg: MessageType::Text(msg.into()),
        }
    }
}

#[derive(Debug)]
pub enum MessageType {
    ConnectReq(String),
    Text(String),
    ConnectWs(HyperWebsocket),
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
