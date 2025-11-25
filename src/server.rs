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

    // Estado compartilhado do jogo
    let chess_match = Arc::new(Mutex::new(ChessMatch::new()));
    
    // Aceita conexão do Jogador 1 (Brancas)
    println!("Waiting for Player 1 (White)...");
    let (mut socket_white, _) = listener.accept().await?;
    println!("Player 1 connected!");

    // Aceita conexão do Jogador 2 (Pretas)
    println!("Waiting for Player 2 (Black)...");
    let (mut socket_black, _) = listener.accept().await?;
    println!("Player 2 connected!");

    // Loop principal
    loop {
        let game = chess_match.lock().await;
        let current_turn = game.get_current_player();
        
        // Prepara mensagem de estado
        let msg = game.to_game_state("".to_string());
        let serialized_msg = serde_json::to_string(&msg).unwrap();

        // Envia estado para ambos
        send_packet(&mut socket_white, &serialized_msg).await?;
        send_packet(&mut socket_black, &serialized_msg).await?;

        // Verifica Game Over
        if game.check_mate {
            println!("Checkmate! Winner: {:?}", current_turn);
            break; 
        }

        // Drop lock para permitir que aguardemos I/O
        drop(game);

        // Aguarda jogada do jogador da vez
        let move_result = if current_turn == Color::White {
             read_packet(&mut socket_white).await?
        } else {
             read_packet(&mut socket_black).await?
        };

        let request: GameMessage = serde_json::from_str(&move_result)?;

        if let GameMessage::MakeMove { source, target } = request {
            let mut game = chess_match.lock().await;
            
            // Tenta processar o movimento
            let s_pos = ChessPosition::from_str(&source);
            let t_pos = ChessPosition::from_str(&target);
            
            if let (Ok(s), Ok(t)) = (s_pos, t_pos) {
                match game.perform_chess_move(s, t) {
                    Ok(_) => println!("Move performed: {} -> {}", source, target),
                    Err(e) => {
                        // Envia erro apenas para o jogador atual (implementação simplificada envia estado atualizado com erro na próxima iteração)
                        println!("Invalid move attempted: {}", e);
                    }
                }
            }
        }
    }
    Ok(())
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