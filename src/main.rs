// Usamos a biblioteca 'rust_chess' para ter acesso a toda a lógica do jogo
use rust_chess::{chess::ChessMatch, ui};
use std::io::{self, Write};

// Função auxiliar para pausar a execução
fn pause_for_user() {
    print!("Press Enter to continue...");
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
}

fn main() {
    // Declara os módulos que este binário específico usa.
    // É importante notar que estamos usando o 'crate', que se refere à biblioteca.
    use rust_chess::chess;
    use rust_chess::ui;

    let mut chess_match = ChessMatch::new();

    // Loop principal do jogo, continua enquanto não houver xeque-mate.
    while !chess_match.check_mate {
        ui::clear_screen();
        ui::print_match(&chess_match);

        // Loop interno para o turno de um jogador.
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
                        Ok(_) => {
                            break; // Movimento válido, sai do loop do turno.
                        }
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
    
    // O jogo acabou. Mostra o resultado final.
    ui::clear_screen();
    ui::print_match(&chess_match);
}