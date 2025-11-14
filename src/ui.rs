use crate::board::piece::Piece;
use crate::board::{position::Position, Board};
use crate::chess::{chess_position::ChessPosition, color::Color, ChessMatch};
use colored::*;
use std::io::{self, Write};
use std::str::FromStr;

pub fn clear_screen() {
    clearscreen::clear().expect("failed to clear screen");
}

pub fn read_chess_position() -> ChessPosition {
    loop {
        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match ChessPosition::from_str(&input) {
            Ok(pos) => return pos,
            Err(e) => {
                println!("Error: {}. Valid values are from a1 to h8.", e);
                print!("Source: ");
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

pub fn print_board(board: &Board, possible_moves: Option<Vec<Vec<bool>>>) {
    for i in 0..board.rows {
        print!("{} ", (8 - i));
        for j in 0..board.cols {
            let pos = Position::new(i, j);
            let background = if let Some(moves) = &possible_moves {
                moves[i][j]
            } else {
                false
            };
            print_piece(board.piece(pos), background);
        }
        println!();
    }
    println!("  a b c d e f g h");
}

fn print_piece(piece: Option<&Box<dyn Piece>>, background: bool) {
    let piece_str = if let Some(p) = piece {
        let color = p.color();
        let symbol = p.to_string();
        if color == Color::White {
            symbol.truecolor(235, 235, 235)
        } else {
            symbol.yellow()
        }
    } else {
        "-".normal()
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
    print!("White: ");
    print!("[");
    print!("{}", white.join(", ").truecolor(235, 235, 235));
    println!("]");

    print!("Black: ");
    print!("[");
    print!("{}", black.join(", ").yellow());
    println!("]");
}