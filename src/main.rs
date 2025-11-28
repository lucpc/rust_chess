// src/main.rs
mod board;
mod chess;
mod error;
mod ui;
mod network;
mod server;
mod client;

use std::env;

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
            // Cliente agora solicita partida ao servidor — não precisa mais de cor no CLI
            let addr = if args.len() > 2 { &args[2] } else { "127.0.0.1:8080" };

            if let Err(e) = client::run_client(addr).await {
                eprintln!("Client error: {}", e);
            }
        }
        _ => {
            println!("Invalid mode. Use 'server' or 'client'.");
        }
    }
}