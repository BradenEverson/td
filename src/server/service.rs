use futures::stream::SplitSink;
use futures_util::{Future, StreamExt};
use http_body_util::Full;
use hyper::{
    body::{self, Bytes},
    service::Service,
    upgrade::Upgraded,
    Method, StatusCode,
};
use hyper::{Request, Response};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, pin::Pin};
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};
use uuid::Uuid;

pub struct ServerService {
    pub sender: UnboundedSender<ServerMessage>,
}

pub type TokioMpscError = tokio::sync::mpsc::error::SendError<ServerMessage>;
pub type WebSocketWriteStream =
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
    pub fn websocket(
        &mut self,
        id: Uuid,
        socket: WebSocketWriteStream,
    ) -> Result<(), TokioMpscError> {
        let message = ServerMessage::new(id, MessageType::ConnectWs(socket));
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
            let (response, websocket) =
                hyper_tungstenite::upgrade(&mut req, None).expect("Error upgrading to WebSocket");
            tokio::spawn(async move {
                match websocket.await {
                    Ok(ws) => {
                        let (writer, mut reader) = ws.split();
                        let user_id = Uuid::new_v4();

                        tx.send(ServerMessage::new(user_id, MessageType::ConnectWs(writer)))
                            .expect("Failed to send websocket write stream up channel");

                        while let Some(Ok(msg)) = reader.next().await {
                            // TODO - Respond to websocket messages accordingly
                            match msg {
                                Message::Text(txt) => {
                                    let parsed: ClientMessage = serde_json::from_str(&txt).unwrap();
                                    match parsed.r#type.as_str() {
                                        "Text" => {
                                            tx.send(ServerMessage::text(user_id, &parsed.data))?
                                        }
                                        "ConnectReq" => tx.send(ServerMessage::new(
                                            user_id,
                                            MessageType::ConnectReq(parsed.data),
                                        ))?,
                                        _ => {}
                                    }
                                }
                                Message::Close(_) => {
                                    println!("Disconnect");
                                    tx.send(ServerMessage::new(user_id, MessageType::Disconnect))?;
                                }
                                _ => {}
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("Failed to establish WebSocket Connection: {}", err)
                    }
                }
                Ok::<(), TokioMpscError>(())
            });

            Box::pin(async { Ok(response) })
        } else {
            // HTTP
            let mut response = Response::builder().status(StatusCode::OK);

            let res = match req.method() {
                &Method::GET => {
                    let path = match req.uri().path() {
                        "/" => "frontend/index.html",
                        "/dist/websocket.js" => {
                            response = response.header("Content-Type", "application/javascript");

                            "frontend/dist/websocket.js"
                        }
                        "/style/styles.css" => {
                            response = response.header("Content-Type", "text/css");

                            "frontend/style/styles.css"
                        }
                        _ => "frontend/404.html",
                    };

                    let page = File::open(path);
                    match page {
                        Ok(mut page) => {
                            let mut buf = vec![];
                            page.read_to_end(&mut buf).expect("Failed to read file");

                            response.body(Full::new(Bytes::copy_from_slice(&buf)))
                        }
                        Err(e) => {
                            panic!("{}{}", e, path);
                        }
                    }
                }
                _ => response.body(Full::new(Bytes::copy_from_slice(&[]))),
            };

            Box::pin(async { res })
        }
    }
}

#[derive(Debug)]
pub struct ServerMessage {
    pub from: Uuid,
    pub msg: MessageType,
}

impl ServerMessage {
    fn new(from: Uuid, msg: MessageType) -> Self {
        Self { from, msg }
    }

    fn text(from: Uuid, msg: &str) -> Self {
        Self::new(from, MessageType::Text(msg.to_string()))
    }
}

#[derive(Debug)]
pub enum MessageType {
    ConnectReq(String),
    Text(String),
    ConnectWs(WebSocketWriteStream),
    BeginGame,
    Disconnect,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerResponse {
    message: ResponseType,
}

impl ServerResponse {
    pub fn new(response: ResponseType) -> Self {
        Self { message: response }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ResponseType {
    Chat(String, String),
    GameStart(Uuid),
    UserJoin(String),
    UserLeave(String),
}

/// Type for interfacing with TypeScript WebSocket
#[derive(Serialize, Deserialize, Debug)]
pub struct ClientMessage {
    pub r#type: String,
    pub data: String,
}
