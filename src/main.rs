use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:0").await.expect("Error starting up the server");

    println!("Listening on port {}", listener.local_addr().unwrap().port());

    loop {
        let (socket, _) = listener.accept().await.expect("Error accepting incoming connection");

        let io = TokioIo::new(socket);

        tokio::spawn(async move {

        });
    }
}

