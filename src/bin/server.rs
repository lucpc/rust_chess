// Use a biblioteca 'rust_chess' para ter acesso a toda a lógica do jogo
use rust_chess::{board, chess, error, ui};
use std::error::Error;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

// Definimos um tipo para nossa fila de matchmaking para facilitar a leitura.
type MatchmakingQueue = Arc<Mutex<Option<TcpStream>>>;

// A função principal agora é assíncrona, marcada com `#[tokio::main]`.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Inicia o listener TCP na porta 8080.
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Chess server listening on 127.0.0.1:8080");

    // Cria a nossa fila de matchmaking compartilhada.
    let waiting_player = Arc::new(Mutex::new(None));

    // Loop principal do servidor para aceitar conexões.
    loop {
        let (stream, addr) = listener.accept().await?;
        println!("Accepted new connection from: {}", addr);

        // Clona o `Arc` para que a nova task também tenha acesso à fila.
        let queue_clone = Arc::clone(&waiting_player);

        // Cria uma nova task para lidar com a lógica de matchmaking para este cliente.
        tokio::spawn(async move {
            handle_connection(stream, queue_clone).await;
        });
    }
}

// Lida com uma nova conexão e tenta encontrar um par.
async fn handle_connection(mut stream: TcpStream, queue: MatchmakingQueue) {
    // Adquire o "lock" da fila.
    let mut waiting_player_lock = queue.lock().await;

    if let Some(opponent_stream) = waiting_player_lock.take() {
        // Se `take()` retorna `Some`, havia um jogador esperando!
        println!(
            "Match found! Starting game between {} and {}",
            stream.peer_addr().unwrap(),
            opponent_stream.peer_addr().unwrap()
        );
        
        // Inicia o loop do jogo em uma nova task.
        tokio::spawn(game_loop(opponent_stream, stream));

    } else {
        // Se a fila estava vazia, este jogador se torna o jogador em espera.
        stream.write_all(b"MSG:Waiting for an opponent...\n").await.unwrap_or_default();
        *waiting_player_lock = Some(stream);
    }
}

// O loop principal para uma partida de xadrez entre dois jogadores.
async fn game_loop(player1: TcpStream, player2: TcpStream) {
    let mut chess_match = chess::ChessMatch::new();

    let mut white_player = BufReader::new(player1);
    let mut black_player = BufReader::new(player2);
    
    // Envia mensagens de boas-vindas.
    white_player.write_all(b"WELCOME:WHITE\n").await.unwrap_or_default();
    black_player.write_all(b"WELCOME:BLACK\n").await.unwrap_or_default();

    loop {
        // Renderiza o tabuleiro para uma string.
        let board_state = ui::render_board_to_string(&chess_match.board, None);
        let status_msg = format!(
            "TURN:{}:{}\n",
            chess_match.get_turn(),
            if chess_match.get_current_player() == chess::color::Color::White { "WHITE" } else { "BLACK" }
        );

        // Envia o estado para os dois jogadores.
        white_player.write_all(board_state.as_bytes()).await.unwrap_or_default();
        white_player.write_all(status_msg.as_bytes()).await.unwrap_or_default();
        black_player.write_all(board_state.as_bytes()).await.unwrap_or_default();
        black_player.write_all(status_msg.as_bytes()).await.unwrap_or_default();
        
        // Determina qual jogador está ativo.
        let (active_player, opponent_player) = if chess_match.get_current_player() == chess::color::Color::White {
            (&mut white_player, &mut black_player)
        } else {
            (&mut black_player, &mut white_player)
        };
        
        active_player.write_all(b"MSG:Your turn. (e.g., e2e4)\n").await.unwrap_or_default();
        opponent_player.write_all(b"MSG:Waiting for opponent's move...\n").await.unwrap_or_default();

        // Lê o movimento do jogador ativo.
        let mut line = String::new();
        match active_player.read_line(&mut line).await {
            Ok(0) | Err(_) => {
                // Conexão fechada ou erro.
                opponent_player.write_all(b"GAMEOVER:Opponent disconnected.\n").await.unwrap_or_default();
                println!("Player disconnected. Game over.");
                return;
            }
            Ok(_) => {
                let move_str = line.trim();
                if move_str.len() == 4 {
                    if let (Ok(source), Ok(target)) = (
                        move_str[0..2].parse::<chess::chess_position::ChessPosition>(),
                        move_str[2..4].parse::<chess::chess_position::ChessPosition>()
                    ) {
                        match chess_match.perform_chess_move(source, target) {
                            Ok(_) => {
                                if chess_match.check_mate {
                                    let winner = chess_match.get_current_player();
                                    let final_board = ui::render_board_to_string(&chess_match.board, None);
                                    let gameover_msg = format!("GAMEOVER:CHECKMATE! Winner is {:?}.\n", winner);

                                    white_player.write_all(final_board.as_bytes()).await.unwrap_or_default();
                                    white_player.write_all(gameover_msg.as_bytes()).await.unwrap_or_default();
                                    black_player.write_all(final_board.as_bytes()).await.unwrap_or_default();
                                    black_player.write_all(gameover_msg.as_bytes()).await.unwrap_or_default();
                                    
                                    println!("Game over: Checkmate!");
                                    return;
                                }
                            }
                            Err(e) => {
                                let error_msg = format!("ERROR:{}\n", e);
                                active_player.write_all(error_msg.as_bytes()).await.unwrap_or_default();
                            }
                        }
                    } else {
                        active_player.write_all(b"ERROR:Invalid move format. Use 'a1h8' format.\n").await.unwrap_or_default();
                    }
                } else {
                    active_player.write_all(b"ERROR:Invalid move format. Use 4 characters like 'e2e4'.\n").await.unwrap_or_default();
                }
            }
        }
    }
}