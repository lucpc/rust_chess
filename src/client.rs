// src/client.rs
use crate::network::GameMessage;
use crate::ui;
use crate::chess::color::Color;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// Versão revisada com cor
pub async fn run_client_with_color(addr: &str, my_color: Color) -> Result<(), Box<dyn std::error::Error>> {
    let mut socket = TcpStream::connect(addr).await?;
    println!("Connected as {:?}!", my_color);

    loop {
        let packet = read_packet(&mut socket).await?;
        let msg: GameMessage = serde_json::from_str(&packet)?;

        match msg {
            GameMessage::GameState { board, turn_color, is_check, is_check_mate, message } => {
                ui::clear_screen();
                ui::print_board(&board, Some(my_color));
                println!("\nMessage: {}", message);
                if is_check { println!("CHECK!"); }
                println!("Turn: {:?}", turn_color);

                if is_check_mate {
                    println!("CHECKMATE! Winner: {:?}", turn_color); // Quem jogou por último ganhou (aprox)
                    break;
                }

                if turn_color == my_color {
                    println!("YOUR TURN!");
                    let source = ui::read_input("Source (e.g., e2): ");
                    let target = ui::read_input("Target (e.g., e4): ");
                    
                    let move_msg = GameMessage::MakeMove { source, target };
                    let serialized = serde_json::to_string(&move_msg).unwrap();
                    send_packet(&mut socket, &serialized).await?;
                } else {
                    println!("Waiting for opponent...");
                }
            }
            _ => {}
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