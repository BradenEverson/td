use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use td::server::service::{ServerMessage, ServerService};
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

    while let Some(msg) = rx.recv().await {
        tokio::spawn(async move {
            // handle incoming message asynchronously
        });
    }
}
