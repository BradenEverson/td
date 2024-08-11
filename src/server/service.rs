use futures_util::Future;
use http_body_util::Full;
use hyper::{
    body::{self, Bytes},
    service::Service,
};
use hyper::{Request, Response};
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;

pub struct ServerService {
    sender: UnboundedSender<ServerMessage>,
}

impl Service<Request<body::Incoming>> for ServerService {
    type Response = Response<Full<Bytes>>;
    type Error = hyper::http::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<body::Incoming>) -> Self::Future {
        todo!()
    }
}

#[derive(Serialize, Deserialize)]
pub struct ServerMessage {
    from: Option<Uuid>,
    msg: MessageType,
}

#[derive(Serialize, Deserialize)]
pub enum MessageType {
    ConnectReq(String),
    Text(String),
    Disconnect,
}
