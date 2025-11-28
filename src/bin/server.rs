use rust_chess::{chess, ui};
use std::error::Error;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

type MatchmakingQueue = Arc<Mutex<Option<TcpStream>>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Chess server listening on 127.0.0.1:8080");

    let waiting_player = Arc::new(Mutex::new(None));

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("Accepted new connection from: {}", addr);

        let queue_clone = Arc::clone(&waiting_player);

        tokio::spawn(async move {
            handle_connection(stream, queue_clone).await;
        });
    }
}

async fn handle_connection(stream: TcpStream, queue: MatchmakingQueue) {
    let mut waiting = queue.lock().await;

    if let Some(opponent_stream) = waiting.take() {
        println!(
            "Match found between {} and {}",
            stream.peer_addr().unwrap(),
            opponent_stream.peer_addr().unwrap()
        );

        tokio::spawn(game_loop(opponent_stream, stream));
    } else {
        let mut writer = BufReader::new(stream);
        let stream_mut = writer.get_mut();

        let _ = stream_mut.write_all(b"MSG:Waiting for opponent...\n").await;
        *waiting = Some(writer.into_inner());
    }
}

async fn game_loop(p1: TcpStream, p2: TcpStream) {
    let mut chess_match = chess::ChessMatch::new();

    let mut white = BufReader::new(p1);
    let mut black = BufReader::new(p2);

    white.get_mut().write_all(b"WELCOME:WHITE\n").await.unwrap_or_default();
    black.get_mut().write_all(b"WELCOME:BLACK\n").await.unwrap_or_default();

    loop {
        let board_state = ui::render_board_to_string(&chess_match.board, None);
        let status = format!(
            "TURN:{}:{}\n",
            chess_match.get_turn(),
            if chess_match.get_current_player() == chess::color::Color::White {
                "WHITE"
            } else {
                "BLACK"
            }
        );

        white.get_mut().write_all(board_state.as_bytes()).await.ok();
        white.get_mut().write_all(status.as_bytes()).await.ok();

        black.get_mut().write_all(board_state.as_bytes()).await.ok();
        black.get_mut().write_all(status.as_bytes()).await.ok();

        let (active, passive) = if chess_match.get_current_player() == chess::color::Color::White {
            (&mut white, &mut black)
        } else {
            (&mut black, &mut white)
        };

        active.get_mut().write_all(b"MSG:Your turn (e2e4)\n").await.ok();
        passive.get_mut().write_all(b"MSG:Waiting...\n").await.ok();

        let input = match read_line(active).await {
            Some(line) => line,
            None => {
                passive.get_mut().write_all(b"GAMEOVER:Opponent disconnected.\n").await.ok();
                return;
            }
        };

        let mv = input.trim();

        if mv.len() != 4 {
            let _ = active.get_mut().write_all(b"ERROR:Use format a1h8\n").await;
            continue;
        }

        let (Ok(source), Ok(target)) = (
            mv[0..2].parse::<chess::chess_position::ChessPosition>(),
            mv[2..4].parse::<chess::chess_position::ChessPosition>(),
        ) else {
            let _ = active.get_mut().write_all(b"ERROR:Invalid move syntax\n").await;
            continue;
        };

        match chess_match.perform_chess_move(source, target) {
            Ok(_) => {
                if chess_match.check_mate {
                    let final_b = ui::render_board_to_string(&chess_match.board, None);
                    let final_msg = format!("GAMEOVER:CHECKMATE! Winner: {:?}\n",
                                            chess_match.get_current_player());
                    let _ = white.get_mut().write_all(final_b.as_bytes()).await;
                    let _ = white.get_mut().write_all(final_msg.as_bytes()).await;
                    let _ = black.get_mut().write_all(final_b.as_bytes()).await;
                    let _ = black.get_mut().write_all(final_msg.as_bytes()).await;
                    return;
                }
            }
            Err(e) => {
                let msg = format!("ERROR:{}\n", e);
                let _ = active.get_mut().write_all(msg.as_bytes()).await;
            }
        }
    }
}

/// Lê 1 linha de um BufReader.
/// Retorna None se o cliente desconectou.
async fn read_line(reader: &mut BufReader<TcpStream>) -> Option<String> {
    let mut buffer = String::new();
    match reader.read_line(&mut buffer).await {
        Ok(0) | Err(_) => None,
        Ok(_) => Some(buffer),
    }
}
