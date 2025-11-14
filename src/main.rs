mod board;
mod chess;
mod error;
mod ui;

use chess::ChessMatch;
use std::io::{self, Write};

fn main() {
    let mut chess_match = ChessMatch::new();

    // Loop principal do jogo, continua enquanto não houver xeque-mate.
    while !chess_match.check_mate {
        ui::clear_screen();
        ui::print_match(&chess_match);

        // Loop interno para o turno de um jogador.
        // Ele só sai deste loop quando um movimento válido é feito.
        loop {
            print!("\nSource: ");
            let source_input = ui::read_chess_position();
            
            // Tenta obter os movimentos possíveis para a peça de origem.
            match chess_match.possible_moves(source_input.to_position()) {
                // SUCESSO: A peça de origem é válida.
                Ok(possible_moves) => {
                    ui::clear_screen();
                    ui::print_board(&chess_match.board, Some(possible_moves));
                    
                    print!("\nTarget: ");
                    let target_input = ui::read_chess_position();

                    // Tenta realizar o movimento.
                    match chess_match.perform_chess_move(source_input, target_input) {
                        // SUCESSO: O movimento completo foi válido.
                        Ok(_) => {
                            break; // Sai do loop do turno e passa para o próximo jogador.
                        }
                        // ERRO: O destino era inválido.
                        Err(e) => {
                            println!("\nError: {}", e);
                            println!("Please try again, starting from the source position.");
                            pause_for_user();
                            // Continua no loop do turno para o jogador tentar de novo.
                        }
                    }
                }
                // ERRO: A peça de origem era inválida.
                Err(e) => {
                    println!("\nError: {}", e);
                    pause_for_user();
                    // Continua no loop do turno para o jogador tentar de novo.
                }
            }
            // Se chegamos aqui, foi por causa de um erro.
            // O loop reiniciará, limpando a tela e mostrando o estado atual do jogo.
            ui::clear_screen();
            ui::print_match(&chess_match);
        }
    }
    
    // O jogo acabou. Mostra o resultado final.
    ui::clear_screen();
    ui::print_match(&chess_match);
}

// Uma função auxiliar para pausar a execução até o usuário pressionar Enter.
fn pause_for_user() {
    print!("Press Enter to continue...");
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
}