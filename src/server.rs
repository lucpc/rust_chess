// src/server.rs
use crate::chess::{ChessMatch, color::Color, chess_position::ChessPosition};
use crate::network::GameMessage;
use std::str::FromStr;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use std::sync::Arc;

pub async fn run_server(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(addr).await?;
    println!("Server listening on {}", addr);

    println!("Server listening on {}", addr);

    // slot de espera para matchmaking: guardamos um socket esperando por oponente
    let waiting: Arc<Mutex<Option<tokio::net::TcpStream>>> = Arc::new(Mutex::new(None));

    loop {
        let (socket, peer) = listener.accept().await?;
        println!("New client connected: {:?}", peer);

        let waiting_clone = waiting.clone();

        // Tenta parear imediatamente: se houver um jogador esperando, pegue-o e crie uma partida
        let mut slot = waiting_clone.lock().await;
        if slot.is_none() {
            // Não há adversário: mande WaitingForOpponent e guarde o socket
            let mut s = socket;
            let waiting_msg = serde_json::to_string(&GameMessage::WaitingForOpponent).unwrap();
            let _ = send_packet(&mut s, &waiting_msg).await;
            *slot = Some(s);
            println!("Player stored in waiting slot — waiting opponent...");
        } else {
            // Há alguém esperando: retire e crie uma partida
            let opponent = slot.take().unwrap();
            println!("Starting a new match between two players...");

            // Spawn uma task para rodar a partida sem bloquear o accept loop
            tokio::spawn(async move {
                if let Err(e) = run_match(opponent, socket).await {
                    eprintln!("Match error: {}", e);
                }
            });
        }
    }
}

async fn send_packet(socket: &mut tokio::net::TcpStream, msg: &str) -> Result<(), std::io::Error> {
    let len = msg.len() as u32;
    socket.write_u32(len).await?;
    socket.write_all(msg.as_bytes()).await?;
    Ok(())
}

async fn read_packet(socket: &mut tokio::net::TcpStream) -> Result<String, std::io::Error> {
    let len = socket.read_u32().await?;
    let mut buf = vec![0u8; len as usize];
    socket.read_exact(&mut buf).await?;
    Ok(String::from_utf8_lossy(&buf).to_string())
}

// Executa o ciclo de jogo para duas conexões — conecta o loop do jogo, envia mensagens e processa jogadas.
async fn run_match(mut socket_a: tokio::net::TcpStream, mut socket_b: tokio::net::TcpStream) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Decide as cores: A = White, B = Black
    let assign_a = serde_json::to_string(&GameMessage::AssignColor(Color::White)).unwrap();
    let assign_b = serde_json::to_string(&GameMessage::AssignColor(Color::Black)).unwrap();

    let _ = send_packet(&mut socket_a, &assign_a).await;
    let _ = send_packet(&mut socket_b, &assign_b).await;

    let mut chess_match = ChessMatch::new();

    loop {
        let current_turn = chess_match.get_current_player();

        // Envia estado para ambos os jogadores
        let state_msg = chess_match.to_game_state("".to_string());
        let serialized = serde_json::to_string(&state_msg).unwrap();
        if let Err(e) = send_packet(&mut socket_a, &serialized).await { eprintln!("Error sending state to A: {}", e); break; }
        if let Err(e) = send_packet(&mut socket_b, &serialized).await { eprintln!("Error sending state to B: {}", e); break; }

        if chess_match.check_mate {
            println!("Match finished (checkmate). Winner: {:?}", current_turn);
            let game_end = serde_json::to_string(&GameMessage::GameEnd { winner: Some(current_turn) }).unwrap();
            let _ = send_packet(&mut socket_a, &game_end).await;
            let _ = send_packet(&mut socket_b, &game_end).await;
            break;
        }

        // Aguarda jogada do jogador da vez
        let result = if current_turn == Color::White {
            read_packet(&mut socket_a).await
        } else {
            read_packet(&mut socket_b).await
        };

        let move_json = match result {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Read error (player disconnected?): {}", e);
                // avisa o outro player que jogo terminou
                let _ = send_packet(&mut socket_a, &serde_json::to_string(&GameMessage::GameEnd { winner: None }).unwrap()).await;
                let _ = send_packet(&mut socket_b, &serde_json::to_string(&GameMessage::GameEnd { winner: None }).unwrap()).await;
                break;
            }
        };

        let request: GameMessage = serde_json::from_str(&move_json)?;

        if let GameMessage::MakeMove { source, target } = request {
            // Tenta aplicar o movimento
            let s_pos = ChessPosition::from_str(&source);
            let t_pos = ChessPosition::from_str(&target);

            if let (Ok(s), Ok(t)) = (s_pos, t_pos) {
                match chess_match.perform_chess_move(s, t) {
                    Ok(_) => println!("Move in match: {} -> {}", source, target),
                    Err(e) => {
                        // Envia erro para o jogador da vez
                        let err_msg = serde_json::to_string(&GameMessage::Error(e.0)).unwrap();
                        if current_turn == Color::White {
                            let _ = send_packet(&mut socket_a, &err_msg).await;
                        } else {
                            let _ = send_packet(&mut socket_b, &err_msg).await;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}