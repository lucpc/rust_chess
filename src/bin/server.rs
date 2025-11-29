use rust_chess::{chess::ChessMatch, ui};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::Mutex;

// Gerador incremental de IDs de sala
static NEXT_ROOM_ID: AtomicU32 = AtomicU32::new(1);

type WaitingRooms = Arc<Mutex<HashMap<u32, TcpStream>>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Servidor de xadrez ouvindo em 127.0.0.1:8080");

    let waiting_rooms: WaitingRooms = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("Nova conexão: {}", addr);

        let rooms_clone = Arc::clone(&waiting_rooms);

        tokio::spawn(async move {
            handle_connection(stream, rooms_clone).await;
        });
    }
}

/// Nova conexão entrando → tenta formar partida
async fn handle_connection(stream: TcpStream, rooms: WaitingRooms) {
    let mut rooms_lock = rooms.lock().await;

    // 1. Pega *qualquer sala com jogador esperando*
    if let Some((&room_id, opponent_stream)) = rooms_lock.iter_mut().next() {
        // remover da hashmap antes de mover
        let opponent = rooms_lock.remove(&room_id).unwrap();

        println!("Sala {} iniciando partida!", room_id);

        // jogador que estava esperando = opponent
        // jogador que chegou agora = stream
        tokio::spawn(game_loop(opponent, stream));
        return;
    }

    // 2. Não havia nenhuma sala esperando → cria nova
    let room_id = NEXT_ROOM_ID.fetch_add(1, Ordering::Relaxed);

    println!("Criando sala {}. Aguardando oponente...", room_id);

    stream
        .try_write(b"MSG:Aguardando oponente...\n")
        .ok();

    rooms_lock.insert(room_id, stream);
}


// ------------------------
//    Função utilitária
// ------------------------
async fn read_line(reader: &mut BufReader<OwnedReadHalf>) -> Option<String> {
    let mut s = String::new();
    match reader.read_line(&mut s).await {
        Ok(0) | Err(_) => None,
        Ok(_) => Some(s.trim().to_string()),
    }
}

// ------------------------
//    Loop principal do jogo
// ------------------------
async fn game_loop(p1: TcpStream, p2: TcpStream) {
    let mut chess_match = ChessMatch::new();

    // Divide streams em leitura + escrita
    let (r1, w1): (OwnedReadHalf, OwnedWriteHalf) = p1.into_split();
    let (r2, w2): (OwnedReadHalf, OwnedWriteHalf) = p2.into_split();

    let mut reader_white = BufReader::new(r1);
    let mut reader_black = BufReader::new(r2);

    let mut writer_white = w1;
    let mut writer_black = w2;

    writer_white.write_all(b"WELCOME:WHITE\n").await.ok();
    writer_black.write_all(b"WELCOME:BLACK\n").await.ok();

    loop {
        let board = ui::render_board_to_string(&chess_match.board, None);
        let turn_msg = format!(
            "TURN:{}:{}\n",
            chess_match.get_turn(),
            if chess_match.get_current_player().is_white() { "WHITE" } else { "BLACK" }
        );

        // Manda tabuleiro + informação
        writer_white.write_all(board.as_bytes()).await.ok();
        writer_white.write_all(turn_msg.as_bytes()).await.ok();
        writer_black.write_all(board.as_bytes()).await.ok();
        writer_black.write_all(turn_msg.as_bytes()).await.ok();

        let (active_reader, active_writer, waiting_writer) =
            if chess_match.get_current_player().is_white() {
                (&mut reader_white, &mut writer_white, &mut writer_black)
            } else {
                (&mut reader_black, &mut writer_black, &mut writer_white)
            };

        active_writer.write_all(b"MSG:Your turn (ex: e2e4)\n").await.ok();
        waiting_writer.write_all(b"MSG:Waiting...\n").await.ok();

        let Some(input) = read_line(active_reader).await else {
            waiting_writer
                .write_all(b"GAMEOVER:Opponent disconnected.\n")
                .await
                .ok();
            return;
        };

        if input.len() != 4 {
            active_writer.write_all(b"ERROR:Use formato a1b2\n").await.ok();
            continue;
        }

        let src = input[0..2].parse();
        let dst = input[2..4].parse();

        match (src, dst) {
            (Ok(a), Ok(b)) => {
                match chess_match.perform_chess_move(a, b) {
                    Ok(_) => {
                        if chess_match.check_mate {
                            let winner = chess_match.get_current_player();
                            let final_board = ui::render_board_to_string(&chess_match.board, None);
                            let msg = format!("GAMEOVER:CHECKMATE! Winner: {:?}\n", winner);

                            writer_white.write_all(final_board.as_bytes()).await.ok();
                            writer_white.write_all(msg.as_bytes()).await.ok();
                            writer_black.write_all(final_board.as_bytes()).await.ok();
                            writer_black.write_all(msg.as_bytes()).await.ok();
                            return;
                        }
                    }
                    Err(e) => {
                        let msg = format!("ERROR:{}\n", e);
                        active_writer.write_all(msg.as_bytes()).await.ok();
                    }
                }
            }
            _ => {
                active_writer.write_all(b"ERROR:Formato invalido\n").await.ok();
            }
        }
    }
}
