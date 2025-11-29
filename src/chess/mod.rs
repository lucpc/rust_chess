// src/chess/mod.rs
pub mod chess_position;
pub mod color;
pub mod pieces;

use crate::board::{piece::Piece, position::Position, Board};
use crate::error::ChessError;
use chess_position::ChessPosition;
use color::Color;
use pieces::{
    bishop::Bishop, king::King, knight::Knight, pawn::Pawn, queen::Queen, rook::Rook,
};
use std::collections::HashSet;
use crate::network::{GameMessage, PieceView}; // Importar

// ... (Mantenha a struct ChessMatch igual)
pub struct ChessMatch {
    pub board: Board,
    turn: u32,
    current_player: Color,
    pub check: bool,
    pub check_mate: bool,
    en_passant_vulnerable: Option<Position>,
    pieces_on_board: HashSet<Position>,
    pub captured_pieces: Vec<Box<dyn Piece + Send + Sync>>,
}

impl ChessMatch {
    // ... (Mantenha o método new e outros getters iguais)
    pub fn new() -> Self {
        // ... (código existente)
        let mut chess_match = ChessMatch {
            board: Board::new(8, 8).unwrap(),
            turn: 1,
            current_player: Color::White,
            check: false,
            check_mate: false,
            en_passant_vulnerable: None,
            pieces_on_board: HashSet::new(),
            captured_pieces: Vec::new(),
        };
        chess_match.initial_setup();
        chess_match
    }
    
    pub fn get_current_player(&self) -> Color {
        self.current_player
    }

    pub fn get_en_passant_vulnerable(&self) -> Option<Position> {
        self.en_passant_vulnerable
    }

    // ADICIONE ESTE MÉTODO NOVO
    pub fn to_game_state(&self, message: String) -> GameMessage {
        let mut board_view = vec![vec![None; 8]; 8];
        
        for r in 0..8 {
            for c in 0..8 {
                let pos = Position::new(r, c);
                if let Some(piece) = self.board.piece(pos) {
                    board_view[r][c] = Some(PieceView {
                        symbol: piece.to_string(),
                        color: piece.color(),
                    });
                }
            }
        }

        GameMessage::GameState {
            board: board_view,
            turn_color: self.current_player,
            is_check: self.check,
            is_check_mate: self.check_mate,
            message,
        }
    }

    // ... (Mantenha calculate_possible_moves, possible_moves, etc. iguais)
    // Certifique-se de que perform_chess_move seja 'pub' (já é no seu código original)
    
    // IMPORTANTE: Copie todo o resto da lógica original (perform_chess_move, validate..., make_move, etc.)
    // Como o arquivo é grande, estou mostrando apenas onde inserir o novo método.
    // O restante do arquivo permanece IDÊNTICO ao original.
    
    fn calculate_possible_moves(&self, source_position: Position) -> Vec<Vec<bool>> {
        self.board
            .piece(source_position)
            .unwrap()
            .possible_moves(&self.board, source_position, self)
    }

    // ... (restante do código original omitido para brevidade, mantenha-o!)
    
    // Apenas para garantir que o compilador não reclame, vou replicar as assinaturas necessárias:
    pub fn perform_chess_move(
        &mut self,
        source: ChessPosition,
        target: ChessPosition,
    ) -> Result<Option<Box<dyn Piece + Send + Sync>>, ChessError> {
        // ... (Código original aqui)
        let source_pos = source.to_position();
        let target_pos = target.to_position();

        self.validate_source_position(source_pos)?;
        self.validate_target_position(source_pos, target_pos)?;

        let captured_piece = self.make_move(source_pos, target_pos);

        if self.test_check(self.current_player) {
            self.undo_move(source_pos, target_pos, captured_piece);
            return Err(ChessError("You can't put yourself in check".to_string()));
        }

        let moved_piece_at_target = self.board.piece(target_pos).unwrap();

        if moved_piece_at_target.to_string().contains("Pawn") || moved_piece_at_target.to_string().contains('♟') || moved_piece_at_target.to_string().contains('♙') {
            if (source_pos.row as isize - target_pos.row as isize).abs() == 2 {
                self.en_passant_vulnerable = Some(target_pos);
            } else {
                self.en_passant_vulnerable = None;
            }
        } else {
            self.en_passant_vulnerable = None;
        }

        let opponent = self.opponent(self.current_player);
        self.check = self.test_check(opponent);

        if self.test_check_mate(opponent) {
            self.check_mate = true;
        } else {
            self.next_turn();
        }

        Ok(captured_piece)
    }
    
    // ... Mantenha as funções privadas auxiliares (validate_source, validate_target, make_move, etc.)
    fn validate_source_position(&self, pos: Position) -> Result<(), ChessError> {
        if let Some(piece) = self.board.piece(pos) {
            if self.current_player != piece.color() {
                return Err(ChessError("The chosen piece is not yours".to_string()));
            }
            if self.calculate_possible_moves(pos).iter().all(|row| row.iter().all(|&x| !x)) {
                return Err(ChessError("There are no possible moves for the chosen piece".to_string()));
            }
        } else {
            return Err(ChessError("There is no piece on source position".to_string()));
        }
        Ok(())
    }

    fn validate_target_position(&self, source: Position, target: Position) -> Result<(), ChessError> {
         if !self.calculate_possible_moves(source)[target.row][target.col] {
            return Err(ChessError("The chosen piece can't move to target position".to_string()));
        }
        Ok(())
    }

    fn make_move(&mut self, source: Position, target: Position) -> Option<Box<dyn Piece + Send + Sync>> {
        // ... (Mesma lógica do original)
        let mut piece = self.board.remove_piece(source).unwrap();
        piece.increase_move_count();
        self.pieces_on_board.remove(&source);

        let mut captured_piece = self.board.remove_piece(target);
        if captured_piece.is_some() {
            self.pieces_on_board.remove(&target);
        }

        let piece_display = piece.to_string(); 
        
        if piece_display.contains('♟') || piece_display.contains('♙') {
            if source.col != target.col && captured_piece.is_none() {
                 let captured_pos = if piece.color() == Color::White {
                    Position::new(target.row + 1, target.col)
                } else {
                    Position::new(target.row - 1, target.col)
                };
                captured_piece = self.board.remove_piece(captured_pos);
                self.pieces_on_board.remove(&captured_pos);
            }
        }

        self.board.place_piece(piece, target).unwrap();
        self.pieces_on_board.insert(target);
        
        // Castling logic (resumida para manter concisão, use a original completa)
        if let Some(moved_piece) = self.board.piece(target) {
             let moved_piece_display = moved_piece.to_string();
             if moved_piece_display.contains('♔') || moved_piece_display.contains('♚') {
                if (target.col as isize - source.col as isize).abs() == 2 {
                    // Implementação do roque igual ao original...
                    if target.col > source.col {
                        let rook_source = Position::new(source.row, source.col + 3);
                        let rook_target = Position::new(source.row, source.col + 1);
                        let mut rook = self.board.remove_piece(rook_source).unwrap();
                        rook.increase_move_count();
                        self.board.place_piece(rook, rook_target).unwrap();
                        self.pieces_on_board.remove(&rook_source);
                        self.pieces_on_board.insert(rook_target);
                    } else {
                        let rook_source = Position::new(source.row, source.col - 4);
                        let rook_target = Position::new(source.row, source.col - 1);
                        let mut rook = self.board.remove_piece(rook_source).unwrap();
                        rook.increase_move_count();
                        self.board.place_piece(rook, rook_target).unwrap();
                        self.pieces_on_board.remove(&rook_source);
                        self.pieces_on_board.insert(rook_target);
                    }
                }
             }
        }

        if let Some(cp) = &captured_piece {
            self.captured_pieces.push(cp.box_clone());
        }

        captured_piece
    }

    fn undo_move(
        &mut self,
        source: Position,
        target: Position,
        captured_piece: Option<Box<dyn Piece + Send + Sync>>,
    ) {
        // 1. Move a peça principal de volta (Target -> Source)
        let mut piece = self.board.remove_piece(target).unwrap();
        piece.decrease_move_count();
        self.board.place_piece(piece, source).unwrap();
        
        // CORREÇÃO: Atualizar o HashSet pieces_on_board
        self.pieces_on_board.remove(&target);
        self.pieces_on_board.insert(source);

        // 2. Restaurar peça capturada (se houver)
        if let Some(cp) = captured_piece {
            let color = cp.color();
            let cp_display = cp.to_string();
            let is_pawn = cp_display.contains('♟') || cp_display.contains('♙');
            
            let mut is_en_passant_capture = false;
            // Verificamos a peça que acabamos de mover de volta para source
            if let Some(p) = self.board.piece(source) {
                let p_display = p.to_string();
                if (p_display.contains('♟') || p_display.contains('♙')) && target.col != source.col {
                    is_en_passant_capture = true;
                }
            }
            
            let place_pos = if is_pawn && is_en_passant_capture {
                if color == Color::White {
                    Position::new(target.row + 1, target.col)
                } else {
                    Position::new(target.row - 1, target.col)
                }
            } else {
                target
            };

            self.board.place_piece(cp, place_pos).unwrap();
            self.captured_pieces.pop();
            
            // CORREÇÃO: Reinserir a peça capturada no HashSet
            self.pieces_on_board.insert(place_pos);
        }

        // 3. Desfazer Roque (se necessário)
        if let Some(moved_piece) = self.board.piece(source) {
            let moved_piece_display = moved_piece.to_string();
            if moved_piece_display.contains('♔') || moved_piece_display.contains('♚') {
                if (target.col as isize - source.col as isize).abs() == 2 {
                    if target.col > source.col {
                        // Roque Pequeno (lado do Rei)
                        let rook_source = Position::new(source.row, source.col + 1);
                        let rook_target = Position::new(source.row, source.col + 3);
                        
                        let mut rook = self.board.remove_piece(rook_source).unwrap();
                        rook.decrease_move_count();
                        self.board.place_piece(rook, rook_target).unwrap();

                        // CORREÇÃO: Atualizar posições da torre
                        self.pieces_on_board.remove(&rook_source);
                        self.pieces_on_board.insert(rook_target);
                    } else {
                        // Roque Grande (lado da Rainha)
                        let rook_source = Position::new(source.row, source.col - 1);
                        let rook_target = Position::new(source.row, source.col - 4);
                        
                        let mut rook = self.board.remove_piece(rook_source).unwrap();
                        rook.decrease_move_count();
                        self.board.place_piece(rook, rook_target).unwrap();

                        // CORREÇÃO: Atualizar posições da torre
                        self.pieces_on_board.remove(&rook_source);
                        self.pieces_on_board.insert(rook_target);
                    }
                }
            }
        }
    } 
    fn test_check(&self, color: Color) -> bool {
        // ... (Mesma lógica do original)
        let king_pos = self.king(color);
        if king_pos.is_none() { return true; }
        let king_pos = king_pos.unwrap();

        let opponent = self.opponent(color);
        let opponent_pieces = self
            .pieces_on_board
            .iter()
            .filter_map(|pos_ref| self.board.piece(*pos_ref).map(|piece| (pos_ref, piece.color())))
            .filter(|&(_, color)| color == opponent)
            .map(|(pos_ref, _)| pos_ref)
            .cloned()
            .collect::<Vec<_>>();

        for pos in opponent_pieces {
            let moves = self.calculate_possible_moves(pos);
            if moves[king_pos.row][king_pos.col] {
                return true;
            }
        }
        false
    }

    fn test_check_mate(&mut self, color: Color) -> bool {
         // ... (Mesma lógica do original)
        if !self.test_check(color) { return false; }

        let player_pieces = self
            .pieces_on_board
            .iter()
            .filter(|&&pos| self.board.piece(pos).unwrap().color() == color)
            .cloned()
            .collect::<Vec<_>>();

        for source_pos in player_pieces {
            let moves = self.calculate_possible_moves(source_pos);
            for r in 0..self.board.rows {
                for c in 0..self.board.cols {
                    if moves[r][c] {
                        let target_pos = Position::new(r, c);
                        let captured = self.make_move(source_pos, target_pos);
                        let still_in_check = self.test_check(color);
                        self.undo_move(source_pos, target_pos, captured);
                        if !still_in_check {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
    
    fn king(&self, color: Color) -> Option<Position> {
         self.pieces_on_board
            .iter()
            .find(|&&pos| {
                let piece = self.board.piece(pos).unwrap();
                let piece_display = piece.to_string();
                (piece_display.contains('♔') || piece_display.contains('♚')) && piece.color() == color
            })
            .cloned()
    }

    fn opponent(&self, color: Color) -> Color {
        if color == Color::White { Color::Black } else { Color::White }
    }

    fn next_turn(&mut self) {
        self.turn += 1;
        self.current_player = self.opponent(self.current_player);
    }
    
    // ... initial_setup e place_new_piece mantidos iguais
    fn place_new_piece(&mut self, pos: ChessPosition, piece: Box<dyn Piece + Send + Sync>) {
        let board_pos = pos.to_position();
        self.board.place_piece(piece, board_pos).unwrap();
        self.pieces_on_board.insert(board_pos);
    }

    fn initial_setup(&mut self) {
         // ... Copie o conteúdo exato do seu initial_setup original aqui ...
         // --- PEÇAS BRANCAS (LINHAS 1 E 2) ---
        self.place_new_piece(ChessPosition::new('a', 1).unwrap(),Box::new(Rook::new(Color::White)));
        self.place_new_piece(ChessPosition::new('b', 1).unwrap(),Box::new(Knight::new(Color::White)));
        self.place_new_piece(ChessPosition::new('c', 1).unwrap(),Box::new(Bishop::new(Color::White)));
        self.place_new_piece(ChessPosition::new('d', 1).unwrap(),Box::new(Queen::new(Color::White)));
        self.place_new_piece(ChessPosition::new('e', 1).unwrap(),Box::new(King::new(Color::White)));
        self.place_new_piece(ChessPosition::new('f', 1).unwrap(),Box::new(Bishop::new(Color::White)));
        self.place_new_piece(ChessPosition::new('g', 1).unwrap(),Box::new(Knight::new(Color::White)));
        self.place_new_piece(ChessPosition::new('h', 1).unwrap(),Box::new(Rook::new(Color::White)));
        for col in 'a'..='h' {
            self.place_new_piece(ChessPosition::new(col, 2).unwrap(),Box::new(Pawn::new(Color::White)));
        }

        // --- PEÇAS PRETAS (LINHAS 7 E 8) ---
        self.place_new_piece(ChessPosition::new('a', 8).unwrap(),Box::new(Rook::new(Color::Black)));
        self.place_new_piece(ChessPosition::new('b', 8).unwrap(),Box::new(Knight::new(Color::Black)));
        self.place_new_piece(ChessPosition::new('c', 8).unwrap(),Box::new(Bishop::new(Color::Black)));
        self.place_new_piece(ChessPosition::new('d', 8).unwrap(),Box::new(Queen::new(Color::Black)));
        self.place_new_piece(ChessPosition::new('e', 8).unwrap(),Box::new(King::new(Color::Black)));
        self.place_new_piece(ChessPosition::new('f', 8).unwrap(),Box::new(Bishop::new(Color::Black)));
        self.place_new_piece(ChessPosition::new('g', 8).unwrap(),Box::new(Knight::new(Color::Black)));
        self.place_new_piece(ChessPosition::new('h', 8).unwrap(),Box::new(Rook::new(Color::Black)));
        for col in 'a'..='h' {
            self.place_new_piece(ChessPosition::new(col, 7).unwrap(),Box::new(Pawn::new(Color::Black)));
        }
    }
}