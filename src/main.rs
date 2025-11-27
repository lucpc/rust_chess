// src/main.rs
mod board;
mod chess;
mod error;
mod ui;

use std::env;
use chess::color::Color;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage:");
        println!("  Run Server: cargo run -- server <address> (default: 127.0.0.1:8080)");
        println!("  Run Client: cargo run -- client <color> <address>");
        println!("Example: cargo run -- client white 127.0.0.1:8080");
        return;
    }

    let mode = &args[1];

    match mode.as_str() {
        "server" => {
            let addr = if args.len() > 2 { &args[2] } else { "127.0.0.1:8080" };
            if let Err(e) = server::run_server(addr).await {
                eprintln!("Server error: {}", e);
            }
        }
        "client" => {
            if args.len() < 3 {
                println!("Please specify color: 'white' or 'black'");
                return;
            }
            let color_str = &args[2].to_lowercase();
            let color = match color_str.as_str() {
                "white" => Color::White,
                "black" => Color::Black,
                _ => {
                    println!("Invalid color. Use 'white' or 'black'.");
                    return;
                }
            };
            
            // O endereço para o cliente é o 4º argumento (índice 3) ou default
            let addr = if args.len() > 3 { &args[3] } else { "127.0.0.1:8080" };
            
            if let Err(e) = client::run_client_with_color(addr, color).await {
                eprintln!("Client error: {}", e);
            }
        }
        _ => {
            println!("Invalid mode. Use 'server' or 'client'.");
        }
    }
}