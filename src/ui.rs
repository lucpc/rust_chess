use crate::board::{self, piece::Piece};
use crate::chess::{chess_position::ChessPosition, color::Color, ChessMatch};
use colored::*;
use std::io::{self, Write};
use std::str::FromStr;

pub fn clear_screen() {
    // expect é aceitável aqui: erro significa ambiente inconsistente.
    clearscreen::clear().expect("failed to clear screen");
}

pub fn read_chess_position() -> ChessPosition {
    loop {
        print!("Source: ");
        
        // flush sem unwrap
        if let Err(e) = io::stdout().flush() {
            eprintln!("Warning: could not flush stdout: {e}");
        }

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                // user sent EOF (Ctrl+D)
                println!("Input closed. Please type a position like 'e2'.");
                continue;
            }
            Ok(_) => {}
            Err(e) => {
                println!("Failed to read line: {e}");
                continue;
            }
        }

        match ChessPosition::from_str(input.trim()) {
            Ok(pos) => return pos,
            Err(e) => {
                println!("Error: {}. Valid values are from a1 to h8.", e);
                continue;
            }
        }
    }
}

pub fn print_match(chess_match: &ChessMatch) {
    print_board(&chess_match.board, None);
    println!();
    print_captured_pieces(&chess_match.captured_pieces);
    println!("\nTurn : {}", chess_match.get_turn());

    if !chess_match.check_mate {
        println!("Waiting player: {:?}", chess_match.get_current_player());
        if chess_match.check {
            println!("{}", "CHECK!".red().bold());
        }
    } else {
        println!("{}", "CHECKMATE!".green().bold());
        println!("Winner: {:?}", chess_match.get_current_player());
    }
}

pub fn print_board(board: &board::Board, possible_moves: Option<Vec<Vec<bool>>>) {
    for i in 0..board.rows {
        print!("{} ", (8 - i));

        for j in 0..board.cols {
            let pos = board::position::Position::new(i, j);

            let background = match &possible_moves {
                Some(moves) if i < moves.len() && j < moves[i].len() => moves[i][j],
                _ => false,
            };

            print_piece(board.piece(pos), background);
        }

        println!();
    }
    println!("  a b c d e f g h");
}

fn print_piece(piece: Option<&Box<dyn Piece>>, background: bool) {
    let piece_str = match piece {
        Some(p) => p.to_string().normal(),
        None => "-".normal(),
    };

    if background {
        print!("{} ", piece_str.on_truecolor(70, 130, 180));
    } else {
        print!("{} ", piece_str);
    }
}

fn print_captured_pieces(captured: &[Box<dyn Piece>]) {
    let white: Vec<String> = captured
        .iter()
        .filter(|p| p.color() == Color::White)
        .map(|p| p.to_string())
        .collect();

    let black: Vec<String> = captured
        .iter()
        .filter(|p| p.color() == Color::Black)
        .map(|p| p.to_string())
        .collect();

    println!("Captured pieces:");
    println!("White: [{}]", white.join(", "));
    println!("Black: [{}]", black.join(", "));
}

// Usado pelo servidor
pub fn render_board_to_string(board: &board::Board, _possible_moves: Option<Vec<Vec<bool>>>) -> String {
    let mut output = String::new();

    for i in 0..board.rows {
        output.push_str(&format!("{} ", (8 - i)));

        for j in 0..board.cols {
            let pos = board::position::Position::new(i, j);

            let piece_str = board
                .piece(pos)
                .map(|p| p.to_string())
                .unwrap_or_else(|| "-".to_string());

            output.push_str(&format!("{} ", piece_str));
        }

        output.push('\n');
    }

    output.push_str("  a b c d e f g h\n");
    output
}
