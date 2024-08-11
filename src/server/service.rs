use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;

pub struct ServerService {
    sender: UnboundedSender<ServerMessage>,
}

pub enum ServerMessage {
    ConnectReq(String),
    Disconnect(Uuid),
}
