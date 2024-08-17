use std::sync::Arc;

use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use td::server::service::{
    MessageType, ResponseType, ServerMessage, ServerResponse, ServerService,
};
use td::server::state::State;
use td::server::user::User;
use tokio::net::TcpListener;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    // TODO: Change port back to 0, fixed for debugging
    let listener = TcpListener::bind("0.0.0.0:7878")
        .await
        .expect("Error starting up the server");

    println!(
        "Listening on port {}",
        listener.local_addr().unwrap().port()
    );

    let (tx, mut rx): (
        UnboundedSender<ServerMessage>,
        UnboundedReceiver<ServerMessage>,
    ) = mpsc::unbounded_channel();

    tokio::spawn(async move {
        loop {
            let (socket, _) = listener
                .accept()
                .await
                .expect("Error accepting incoming connection");

            let io = TokioIo::new(socket);

            let server_service = ServerService::new(tx.clone());

            tokio::spawn(async move {
                if let Err(e) = http1::Builder::new()
                    .serve_connection(io, server_service)
                    .with_upgrades()
                    .await
                {
                    eprintln!("Error serving connection: {}", e);
                }
            });
        }
    });

    let state = Arc::new(RwLock::new(State::default()));
    while let Some(msg) = rx.recv().await {
        let state_clone = state.clone();
        tokio::spawn(async move {
            let state = state_clone.clone();
            // handle incoming message asynchronously
            println!("{:?}", msg);
            match msg.msg {
                MessageType::Text(txt) => {
                    let name = match state.read().await.get_name(msg.from) {
                        Some(name) => name.clone(),
                        None => "".to_string(),
                    };
                    let response = ServerResponse::new(ResponseType::Chat(name.into(), txt));
                    state
                        .write()
                        .await
                        .broadcast(response)
                        .await
                        .expect("Failed to broadcast to all users");
                }
                MessageType::ConnectWs(ws) => {
                    println!("User Connected with ID {}", msg.from);
                    let mut user = User::default();
                    user.set_id(msg.from);
                    user.set_socket(ws);

                    state.write().await.connect(msg.from, user);
                }
                MessageType::ConnectReq(name) => state.write().await.set_name(msg.from, name),
                MessageType::Disconnect => {
                    state.write().await.disconnect(msg.from);
                }
            }
        });
    }
}
