use rust_chess::{chess::ChessMatch, ui};
use std::io::{self, Write};

/// Pausa execução aguardando Enter, mas sem panicar se der erro.
fn pause_for_user() {
    print!("Press Enter to continue...");

    if let Err(e) = io::stdout().flush() {
        eprintln!("Warning: could not flush stdout: {e}");
    }

    let mut buffer = String::new();

    if let Err(e) = io::stdin().read_line(&mut buffer) {
        eprintln!("Warning: failed to read user input: {e}");
    }
}

fn main() {
    let mut chess_match = ChessMatch::new();

    // Loop principal do jogo local
    while !chess_match.check_mate {
        ui::clear_screen();
        ui::print_match(&chess_match);

        loop {
            print!("\nSource: ");
            let source_input = ui::read_chess_position();

            match chess_match.possible_moves(source_input.to_position()) {
                Ok(possible_moves) => {
                    ui::clear_screen();
                    ui::print_board(&chess_match.board, Some(possible_moves));

                    print!("\nTarget: ");
                    let target_input = ui::read_chess_position();

                    match chess_match.perform_chess_move(source_input, target_input) {
                        Ok(_) => break, // movimento válido
                        Err(e) => {
                            println!("\nError: {}", e);
                            println!("Please try again, starting from the source position.");
                            pause_for_user();
                        }
                    }
                }
                Err(e) => {
                    println!("\nError: {}", e);
                    pause_for_user();
                }
            }

            ui::clear_screen();
            ui::print_match(&chess_match);
        }
    }

    // Fim da partida
    ui::clear_screen();
    ui::print_match(&chess_match);
}
