// src/client.rs
use crate::network::GameMessage;
use crate::ui;
use crate::chess::color::Color;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// Versão revisada com cor
pub async fn run_client(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut socket = TcpStream::connect(addr).await?;
    println!("Connected to server at {}. Waiting for match...", addr);

    let mut my_color: Option<Color> = None;

    loop {
        let packet = match read_packet(&mut socket).await {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Connection lost: {}", e);
                break;
            }
        };

        let msg: GameMessage = serde_json::from_str(&packet)?;

        match msg {
            GameMessage::AssignColor(color) => {
                my_color = Some(color);
                println!("Assigned color: {:?}", my_color.unwrap());
            }

            GameMessage::WaitingForOpponent => {
                println!("Waiting for opponent to join... (server)");
            }

            GameMessage::GameState { board, turn_color, is_check, is_check_mate, message } => {
                ui::clear_screen();
                ui::print_board(&board, my_color);
                println!("\nMessage: {}", message);
                if is_check { println!("CHECK!"); }
                println!("Turn: {:?}", turn_color);

                if is_check_mate {
                    println!("CHECKMATE! Winner: {:?}", turn_color);
                    break;
                }

                // Só permite mover se já soubermos a cor
                if let Some(my_color) = my_color {
                    if turn_color == my_color {
                        println!("YOUR TURN ({:?})!", my_color);
                        let source = ui::read_input("Source (e.g., e2): ");
                        let target = ui::read_input("Target (e.g., e4): ");

                        let move_msg = GameMessage::MakeMove { source, target };
                        let serialized = serde_json::to_string(&move_msg).unwrap();
                        send_packet(&mut socket, &serialized).await?;
                    } else {
                        println!("Waiting for opponent...");
                    }
                } else {
                    println!("Server hasn't assigned your color yet — waiting...");
                }
            }

            GameMessage::GameEnd { winner } => {
                println!("Game finished. Winner: {:?}", winner);
                break;
            }

            GameMessage::Error(err) => {
                eprintln!("Server error: {}", err);
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