// src/ui.rs
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

pub fn print_board(
    board: &Vec<Vec<Option<PieceView>>>,
    my_color: Option<Color>,
    captured_by_white: &Vec<PieceView>,
    captured_by_black: &Vec<PieceView>,
) {
    let perspective_white = my_color.unwrap_or(Color::White) == Color::White;

    // Cabeçalho de peças capturadas
    print!("\n{} ", "Brancas capturaram:".green());
    print_captured_from_views(captured_by_white);
    println!();

    // Tabuleiro com borda fixa
    println!("  {}", "┌─────────────────┐");

    let rows: Vec<usize> = if perspective_white {
        (0..8).collect()
    } else {
        (0..8).rev().collect()
    };

    for i in rows {
        print!("{} {} ", (8 - i).to_string(), "│");
        for j in 0..8 {
            let piece = &board[i][j];
            print_piece(piece);
        }
        println!("{} {}", "│", (8 - i).to_string());
    }

    println!("  {}", "└─────────────────┘");
    println!("    a b c d e f g h");

    // Peças capturadas pelo outro jogador
    print!("\n{} ", "Pretas capturaram:".green());
    print_captured_from_views(captured_by_black);
    println!("\n");
}

fn print_piece(piece: &Option<PieceView>) {
    let piece_str = if let Some(p) = piece {
        let symbol = &p.symbol;
        if p.color == Color::White {
            symbol.bright_white()
        } else {
            symbol.bright_black().cyan()
        }
    } else {
        "·".truecolor(100, 100, 100)
    };
    print!("{} ", piece_str);
}

fn print_captured_from_views(captured: &[PieceView]) {
    if captured.is_empty() {
        print!("{}", "(nenhuma)".truecolor(150, 150, 150));
        return;
    }

    for pv in captured {
        let symbol = &pv.symbol;
        if pv.color == Color::White {
            print!("{} ", symbol.bright_white());
        } else {
            print!("{} ", symbol.bright_black());
        }
    }
}