// src/client.rs
use crate::network::GameMessage;
use crate::ui;
use crate::chess::color::Color;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::{timeout, Duration};
use std::io::{self, Write};

const AMARELO: &str = "\x1b[33m";
const CIANO: &str = "\x1b[36m";
const RESET: &str = "\x1b[0m";
const VERDE: &str = "\x1b[32m";

pub async fn run_client(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut socket = TcpStream::connect(addr).await?;
    println!("Connected to server at {}", addr);
    
    let mut my_color: Option<Color> = None;
    let mut waiting_for_opponent = false;
    let mut frame_index: usize = 0;
    const NUM_FRAMES: usize = 6;
    
    loop {
    
        let packet_result = if waiting_for_opponent {
            show_lobby_frame(frame_index);
            frame_index = (frame_index + 1) % NUM_FRAMES;
            
            timeout(Duration::from_millis(150), read_packet(&mut socket)).await
        } else {
            Ok(read_packet(&mut socket).await)
        };
        
        let packet = match packet_result {
            Ok(Ok(p)) => p,
            Ok(Err(e)) => {
                eprintln!("\nConnection lost: {}", e);
                break;
            }
            Err(_) => {
                continue;
            }
        };
        
        let msg: GameMessage = serde_json::from_str(&packet)?;
        
        match msg {
            GameMessage::AssignColor(color) => {
                my_color = Some(color);
                
                print!("\r{:80}\r", "");
                io::stdout().flush().unwrap();
                println!("{}✓ Assigned color: {:?}{}", VERDE, color, RESET);
            }
            
            GameMessage::WaitingForOpponent => {
                if !waiting_for_opponent {
                    waiting_for_opponent = true;
                    print!("\n");
                }
            }
            
            GameMessage::GameState { board, turn_color, is_check, is_check_mate, message, captured_by_white, captured_by_black } => {
                // Parar de aguardar - jogo começou
                if waiting_for_opponent {
                    waiting_for_opponent = false;
                    print!("\r{:80}\r", ""); // Limpa a linha do lobby
                    io::stdout().flush().unwrap();
                    println!("\n{}╔════════════════════════════════╗", CIANO);
                    println!("║   🎮 Partida iniciada! 🎮      ║");
                    println!("╚════════════════════════════════╝{}\n", RESET);
                    std::thread::sleep(Duration::from_secs(1));
                }
                
                ui::clear_screen();
                ui::print_board(&board, my_color, &captured_by_white, &captured_by_black);
                println!("\n{}", message);
                
                if is_check { 
                    println!("\n{}⚠️  CHECK! ⚠️{}", AMARELO, RESET); 
                }
                
                println!("\nTurn: {:?}", turn_color);
                
                if is_check_mate {
                    println!("\n{}🏆 CHECKMATE! Winner: {:?} 🏆{}", VERDE, turn_color, RESET);
                    break;
                }
                
                if let Some(my_color) = my_color {
                    if turn_color == my_color {
                        println!("\n{}▶ YOUR TURN ({:?})!{}", AMARELO, my_color, RESET);
                        let source = ui::read_input("Source (e.g., e2): ");
                        let target = ui::read_input("Target (e.g., e4): ");
                        let move_msg = GameMessage::MakeMove { source, target };
                        let serialized = serde_json::to_string(&move_msg).unwrap();
                        send_packet(&mut socket, &serialized).await?;
                    } else {
                        println!("\n⏳ Waiting for opponent...");
                    }
                } else {
                    println!("\n⏳ Server hasn't assigned your color yet...");
                }
            }
            
            GameMessage::GameEnd { winner } => {
                println!("\n{}🏁 Game finished. Winner: {:?}{}", CIANO, winner, RESET);
                break;
            }
            
            GameMessage::Error(err) => {
                eprintln!("\n{}❌ Server error: {}{}", AMARELO, err, RESET);
            }
            
            _ => {}
        }
    }
    
    Ok(())
}

fn show_lobby_frame(frame_index: usize) {
    const SPINNER_FRAMES: [&str; 6] = ["♟", "♞", "♝", "♜", "♛", "♚"];
    
    let spinner = SPINNER_FRAMES[frame_index];
    let msg = format!("  {}Buscando adversário{} {} {}", 
                     AMARELO, CIANO, spinner, RESET);
    
    print!("\r{}", msg);
    io::stdout().flush().unwrap();
}

async fn send_packet(socket: &mut TcpStream, msg: &str) -> Result<(), std::io::Error> {
    let len = msg.len() as u32;
    socket.write_u32(len).await?;
    socket.write_all(msg.as_bytes()).await?;
    Ok(())
}

async fn read_packet(socket: &mut TcpStream) -> Result<String, std::io::Error> {
    let len = socket.read_u32().await?;
    let mut buf = vec![0u8; len as usize];
    socket.read_exact(&mut buf).await?;
    Ok(String::from_utf8_lossy(&buf).to_string())
}