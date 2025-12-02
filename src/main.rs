// src/main.rs
mod board;
mod chess;
mod error;
mod ui;
mod network;
mod server;
mod client;

use std::env;

fn show_banner() {
    println!(r#"
        ♜════════════════════════════════════════════════════════♞
        ║                                                        ║
        ║         ██████╗ ██╗  ██╗ █████╗ ███████╗███████╗       ║
        ║        ██╔════╝ ██║  ██║██╔══██╗██╔════╝██╔════╝       ║
        ║        ██║      ███████║███████║███████╗███████╗       ║
        ║        ██║      ██╔══██║██╔══██║╚════██║╚════██║       ║
        ║        ╚██████╗ ██║  ██║██║  ██║███████║███████║       ║
        ║         ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚══════╝       ║ 
        ║                                                        ║
        ║                                                        ║
        ║      ♞ A multiplayer chess game in the terminal ♜      ║
        ║                                                        ║
        ║                                                        ║
        ♚════════════════════════════════════════════════════════♛

    "#);
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        show_banner();
        println!("\nUsage:");
        println!("  Run Server: cargo run -- server <address> (default: 127.0.0.1:8080)");
        println!("  Run Client: cargo run -- client <address>");
        return;
    }

    let mode = &args[1];

    match mode.as_str() {
        "server" => {
            show_banner();
            let addr = if args.len() > 2 { &args[2] } else { "127.0.0.1:8080" };
            if let Err(e) = server::run_server(addr).await {
                eprintln!("Server error: {}", e);
            }
        }
        "client" => {
            show_banner();
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