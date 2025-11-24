use crate::chess::color::Color;
use crate::network::PieceView;
use colored::*;
use std::io::{self, Write};

pub fn clear_screen() {
    clearscreen::clear().expect("failed to clear screen");
}

pub fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

// Alterado para aceitar a matriz de PieceView em vez de &Board
pub fn print_board(board: &Vec<Vec<Option<PieceView>>>, my_color: Option<Color>) {
    let perspective_white = my_color.unwrap_or(Color::White) == Color::White;

    println!("  a b c d e f g h");
    
    let rows: Vec<usize> = if perspective_white {
        (0..8).collect()
    } else {
        (0..8).rev().collect()
    };

    for i in rows {
        print!("{} ", (8 - i));
        for j in 0..8 {
            let piece = &board[i][j];
            print_piece(piece);
        }
        println!(" {}", (8 - i));
    }
    println!("  a b c d e f g h");
}

fn print_piece(piece: &Option<PieceView>) {
    let piece_str = if let Some(p) = piece {
        let symbol = &p.symbol;
        if p.color == Color::White {
            symbol.truecolor(235, 235, 235)
        } else {
            symbol.yellow()
        }
    } else {
        "-".normal()
    };
    print!("{} ", piece_str);
}