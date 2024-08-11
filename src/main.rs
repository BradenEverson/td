use hyper_util::rt::TokioIo;
use td::server::service::ServerMessage;
use tokio::net::TcpListener;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:0")
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

            tokio::spawn(async move {});
        }
    });

    while let Some(msg) = rx.recv().await {}
}
