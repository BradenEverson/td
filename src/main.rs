use std::sync::Arc;

use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use td::game::card_gen::UNITS;
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
                    let response = ServerResponse::new(ResponseType::Chat(name, txt));
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
                MessageType::ConnectReq(name) => {
                    let response = ServerResponse::new(ResponseType::UserJoin(name.clone()));
                    {
                        let mut state = state.write().await;
                        state
                            .broadcast(response)
                            .await
                            .expect("Failed to broadcast to all users");

                        state.set_name(msg.from, name)
                    }
                }
                MessageType::Disconnect => {
                    let mut state = state.write().await;

                    let possible_enemy = state.get_opponent(msg.from);

                    if let Some(enemy) = possible_enemy {
                        let win_by_default =
                            ServerResponse::new(ResponseType::WinByDisconnect(msg.from));
                        state
                            .broadcast_to(win_by_default, &[enemy])
                            .await
                            .expect("Failed to broadcast to user")
                    }

                    let name = state.get_name(msg.from);

                    if let Some(name) = name {
                        let response = ServerResponse::new(ResponseType::UserLeave(name.clone()));
                        state
                            .broadcast_to_all_but(response, &[msg.from])
                            .await
                            .expect("Error broadcasting to all users");
                    }
                    state.disconnect(msg.from);
                }
                MessageType::BeginGame => {
                    let mut state = state.write().await;
                    let result = state.new_random(msg.from);

                    match result {
                        Ok((_battle_id, against)) => {
                            let name_a = state.get_name(msg.from).unwrap().clone();
                            let name_b = state.get_name(against).unwrap().clone();

                            let message_a = ServerResponse::new(ResponseType::StartGame(
                                name_a.clone(),
                                name_b.clone(),
                            ));
                            let message_b =
                                ServerResponse::new(ResponseType::StartGame(name_b, name_a));

                            state
                                .broadcast_to(message_a, &[against])
                                .await
                                .expect("Failed to broadcast message");
                            state
                                .broadcast_to(message_b, &[msg.from])
                                .await
                                .expect("Failed to broadcast message");

                            state
                                .broadcast_users_hand(against)
                                .await
                                .expect("Failed to send hand");
                            state
                                .broadcast_users_hand(msg.from)
                                .await
                                .expect("Failed to send hand");
                        }
                        Err(error) => {
                            panic!("Starting game failed: {}\n\nPotential TODO: map error and if its a lobby full error broadcast that to the user", error);
                        }
                    }
                }
                MessageType::PlayUnit(card_name) => {
                    let unit = *UNITS
                        .iter()
                        .find(|unit| unit.get_name() == card_name)
                        .expect("Unit doesn't exist :/");

                    let response_back =
                        ServerResponse::new(ResponseType::UnitSpawned(true, Box::new(unit)));
                    let response_to_opponent =
                        ServerResponse::new(ResponseType::UnitSpawned(false, Box::new(unit)));
                    let opponent = state
                        .read()
                        .await
                        .get_opponent(msg.from)
                        .expect("Couldn't find opponent");

                    let mut state = state.write().await;
                    state
                        .broadcast_to(response_back, &[msg.from])
                        .await
                        .expect("Failed to broadcast message");

                    state
                        .broadcast_to(response_to_opponent, &[opponent])
                        .await
                        .expect("Failed to broadcast message");
                }
                MessageType::DmgPing(dmg) => {
                    let opponent = {
                        state
                            .read()
                            .await
                            .get_opponent(msg.from)
                            .expect("User wasn't in a battle but expected an opponent")
                    };

                    let mut state = state.write().await;
                    let result = state.damage(opponent, dmg);

                    match result {
                        Some(remaining_hp) => {
                            let health_update_to_sender = ServerResponse::new(
                                ResponseType::NewTowerHealth(false, remaining_hp),
                            );
                            let health_update_to_enemy = ServerResponse::new(
                                ResponseType::NewTowerHealth(true, remaining_hp),
                            );

                            state
                                .broadcast_to(health_update_to_sender, &[msg.from])
                                .await
                                .expect("Failed to broadcast message back to sender");

                            state
                                .broadcast_to(health_update_to_enemy, &[opponent])
                                .await
                                .expect("Failed to broadcast message back to sender");
                        }
                        None => {
                            let win = ServerResponse::new(ResponseType::Win(msg.from));
                            let lose = ServerResponse::new(ResponseType::Lose(opponent));

                            state
                                .broadcast_to(win, &[msg.from])
                                .await
                                .expect("Failed to broadcast message back to sender");

                            state
                                .broadcast_to(lose, &[opponent])
                                .await
                                .expect("Failed to broadcast message back to sender");
                        }
                    }
                }
            }
        });
    }
}
