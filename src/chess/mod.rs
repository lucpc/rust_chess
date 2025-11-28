pub mod chess_position;
pub mod color;
pub mod pieces;

use crate::board::{piece::Piece, position::Position, Board};
use crate::error::ChessError;

use self::chess_position::ChessPosition;
use self::color::Color;
use self::pieces::{bishop::Bishop, king::King, knight::Knight, pawn::Pawn, queen::Queen, rook::Rook};

use std::collections::HashSet;

/// Representa o estado completo de uma partida de xadrez.
pub struct ChessMatch {
    pub board: Board,
    turn: u32,
    current_player: Color,
    pub check: bool,
    pub check_mate: bool,
    en_passant_vulnerable: Option<Position>,
    pieces_on_board: HashSet<Position>,
    pub captured_pieces: Vec<Box<dyn Piece>>,
}

impl ChessMatch {
    /// Cria uma nova partida de xadrez com o tabuleiro inicial configurado.
    pub fn new() -> Self {
        // 8x8 é um tamanho válido de tabuleiro; se falhar aqui é bug de programação,
        // não erro de entrada do usuário.
        let board = Board::new(8, 8).expect("Falha ao criar tabuleiro 8x8: configuração inválida");

        let mut chess_match = ChessMatch {
            board,
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

    pub fn get_turn(&self) -> u32 {
        self.turn
    }

    pub fn get_current_player(&self) -> Color {
        self.current_player
    }

    pub fn get_en_passant_vulnerable(&self) -> Option<Position> {
        self.en_passant_vulnerable
    }

    /// Acessa uma peça no tabuleiro em posição conhecida como válida e ocupada.
    /// 
    /// Esta função é usada somente quando a lógica já garantiu que:
    /// - a posição existe no tabuleiro; e
    /// - há uma peça nessa posição.
    ///
    /// Se falhar, é considerado bug interno (invariante quebrado).
    fn piece_at(&self, pos: Position) -> &Box<dyn Piece> {
        self.board
            .piece(pos)
            .expect("Invariante quebrada: esperado haver peça nessa posição")
    }

    /// Calcula movimentos possíveis *brutos* de uma peça (sem checar se o movimento
    /// deixa o próprio rei em xeque).
    fn calculate_possible_moves(&self, source_position: Position) -> Vec<Vec<bool>> {
        let piece = self.piece_at(source_position);
        piece.possible_moves(&self.board, source_position, self)
    }

    /// Retorna todos os movimentos possíveis para a peça na posição de origem,
    /// validando se:
    /// - existe uma peça na origem,
    /// - a peça é do jogador atual,
    /// - há ao menos um movimento legal para ela.
    pub fn possible_moves(&self, source_position: Position) -> Result<Vec<Vec<bool>>, ChessError> {
        self.validate_source_position(source_position)?;
        Ok(self.calculate_possible_moves(source_position))
    }

    /// Executa um movimento de xadrez completo (com validações) a partir de posições
    /// em notação de xadrez (`ChessPosition`), retornando a peça capturada (se houver).
    pub fn perform_chess_move(
        &mut self,
        source: ChessPosition,
        target: ChessPosition,
    ) -> Result<Option<Box<dyn Piece>>, ChessError> {
        let source_pos = source.to_position();
        let target_pos = target.to_position();

        // Valida origem e destino do movimento
        self.validate_source_position(source_pos)?;
        self.validate_target_position(source_pos, target_pos)?;

        let captured_piece = self.make_move(source_pos, target_pos);

        // Se após o movimento o jogador atual permanecer em xeque,
        // o movimento é inválido e deve ser desfeito.
        if self.test_check(self.current_player) {
            self.undo_move(source_pos, target_pos, captured_piece);
            return Err(ChessError("You can't put yourself in check".to_string()));
        }

        // Atualiza estado de vulnerabilidade para en passant
        self.update_en_passant_state(source_pos, target_pos);

        // Verifica xeque e xeque-mate
        let opponent = self.opponent(self.current_player);
        self.check = self.test_check(opponent);

        if self.test_check_mate(opponent) {
            self.check_mate = true;
        } else {
            self.next_turn();
        }

        Ok(captured_piece)
    }

    /// Garante que há uma peça válida na posição de origem e que ela pertence ao
    /// jogador atual e possui movimentos possíveis.
    fn validate_source_position(&self, pos: Position) -> Result<(), ChessError> {
        let maybe_piece = self.board.piece(pos);

        let piece = match maybe_piece {
            Some(p) => p,
            None => {
                return Err(ChessError(
                    "There is no piece on source position".to_string(),
                ))
            }
        };

        if self.current_player != piece.color() {
            return Err(ChessError("The chosen piece is not yours".to_string()));
        }

        let possible_moves = self.calculate_possible_moves(pos);
        let has_any_move = possible_moves
            .iter()
            .any(|row| row.iter().any(|&can_move| can_move));

        if !has_any_move {
            return Err(ChessError(
                "There are no possible moves for the chosen piece".to_string(),
            ));
        }

        Ok(())
    }

    /// Garante que a posição de destino é alcançável a partir da origem,
    /// segundo os movimentos possíveis da peça.
    fn validate_target_position(
        &self,
        source: Position,
        target: Position,
    ) -> Result<(), ChessError> {
        let moves = self.calculate_possible_moves(source);

        if target.row >= self.board.rows || target.col >= self.board.cols {
            return Err(ChessError(
                "Target position is out of the board".to_string(),
            ));
        }

        if !moves[target.row][target.col] {
            return Err(ChessError(
                "The chosen piece can't move to target position".to_string(),
            ));
        }

        Ok(())
    }

    /// Atualiza estado de en passant baseado no último movimento de peão.
    fn update_en_passant_state(&mut self, source: Position, target: Position) {
        let moved_piece = self.board.piece(target);

        // Se não houver peça no destino (o que não deveria acontecer aqui), apenas limpa o estado.
        let Some(piece) = moved_piece else {
            self.en_passant_vulnerable = None;
            return;
        };

        let piece_display = piece.to_string();
        let is_pawn = piece_display.contains('♟') || piece_display.contains('♙');

        if is_pawn {
            // Se o peão se moveu duas casas, ele fica vulnerável ao en passant
            let row_diff = (source.row as isize - target.row as isize).abs();
            if row_diff == 2 {
                self.en_passant_vulnerable = Some(target);
            } else {
                self.en_passant_vulnerable = None;
            }
        } else {
            self.en_passant_vulnerable = None;
        }
    }

    /// Executa efetivamente o movimento no tabuleiro, incluindo:
    /// - captura normal,
    /// - captura en passant,
    /// - roque.
    ///
    /// Retorna a peça capturada, se houver.
    fn make_move(&mut self, source: Position, target: Position) -> Option<Box<dyn Piece>> {
        // Remove peça da origem
        let mut moving_piece = self
            .board
            .remove_piece(source)
            .expect("Invariante quebrada: esperado haver peça na origem ao mover");
        moving_piece.increase_move_count();
        self.pieces_on_board.remove(&source);

        // Captura (se houver) na casa de destino
        let mut captured_piece = self.board.remove_piece(target);
        if captured_piece.is_some() {
            self.pieces_on_board.remove(&target);
        }

        // Lida com captura en passant (peão movendo na diagonal sem peça aparente no destino)
        {
            let piece_display = moving_piece.to_string();
            let is_pawn = piece_display.contains('♟') || piece_display.contains('♙');

            if is_pawn && source.col != target.col && captured_piece.is_none() {
                // Captura en passant
                let captured_pos = if moving_piece.color() == Color::White {
                    Position::new(target.row + 1, target.col)
                } else {
                    Position::new(target.row - 1, target.col)
                };

                captured_piece = self.board.remove_piece(captured_pos);
                if captured_piece.is_some() {
                    self.pieces_on_board.remove(&captured_pos);
                }
            }
        }

        // Coloca peça na casa de destino
        self.board
            .place_piece(moving_piece, target)
            .expect("Invariante quebrada: falha ao colocar peça em posição destino");
        self.pieces_on_board.insert(target);

        // Lida com roque (rei movendo duas casas)
        self.handle_castling(source, target);

        // Registra peça capturada para exibição posterior
        if let Some(ref cp) = captured_piece {
            self.captured_pieces.push(cp.box_clone());
        }

        captured_piece
    }

    /// Desfaz um movimento previamente realizado por `make_move`,
    /// incluindo desfazer:
    /// - captura normal,
    /// - captura en passant,
    /// - roque.
    fn undo_move(&mut self, source: Position, target: Position, captured_piece: Option<Box<dyn Piece>>) {
        // Volta a peça que se moveu para a origem
        let mut moving_piece = self
            .board
            .remove_piece(target)
            .expect("Invariante quebrada: esperado haver peça no destino ao desfazer movimento");
        moving_piece.decrease_move_count();
        self.board
            .place_piece(moving_piece, source)
            .expect("Invariante quebrada: falha ao recolocar peça na origem");

        // Se havia captura, reposiciona a peça capturada
        if let Some(captured) = captured_piece {
            let captured_color = captured.color();
            let captured_display = captured.to_string();
            let captured_is_pawn = captured_display.contains('♟') || captured_display.contains('♙');

            // Verifica se era uma captura en passant
            let is_en_passant_capture = {
                if let Some(piece_at_source) = self.board.piece(source) {
                    let display = piece_at_source.to_string();
                    let is_pawn = display.contains('♟') || display.contains('♙');
                    is_pawn && target.col != source.col
                } else {
                    false
                }
            };

            let place_pos = if captured_is_pawn && is_en_passant_capture {
                // Recoloca o peão capturado en passant atrás da casa de destino
                if captured_color == Color::White {
                    Position::new(target.row + 1, target.col)
                } else {
                    Position::new(target.row - 1, target.col)
                }
            } else {
                target
            };

            self.board
                .place_piece(captured, place_pos)
                .expect("Invariante quebrada: falha ao recolocar peça capturada");

            // Remove última entrada em `captured_pieces`, se houver
            self.captured_pieces.pop();
        }

        // Desfaz roque, caso tenha ocorrido
        self.undo_castling(source, target);
    }

    /// Verifica se um determinado lado está em xeque.
    fn test_check(&self, color: Color) -> bool {
        // Se não achar o rei, consideramos que está em "estado inválido",
        // e tratamos como xeque (situação extrema).
        let king_pos = match self.king(color) {
            Some(pos) => pos,
            None => return true,
        };

        let opponent = self.opponent(color);

        let opponent_positions: Vec<Position> = self
            .pieces_on_board
            .iter()
            .copied()
            .filter(|&pos| {
                self.board
                    .piece(pos)
                    .map(|p| p.color() == opponent)
                    .unwrap_or(false)
            })
            .collect();

        for pos in opponent_positions {
            let moves = self.calculate_possible_moves(pos);
            if moves[king_pos.row][king_pos.col] {
                return true;
            }
        }

        false
    }

    /// Verifica se um determinado lado está em xeque-mate.
    ///
    /// A lógica é:
    /// 1. Se não estiver em xeque, não é xeque-mate.
    /// 2. Para cada peça do jogador:
    ///    - Para cada movimento possível:
    ///      - Simula o movimento.
    ///      - Se algum movimento tirar o rei do xeque, não é xeque-mate.
    fn test_check_mate(&mut self, color: Color) -> bool {
        if !self.test_check(color) {
            return false;
        }

        let player_positions: Vec<Position> = self
            .pieces_on_board
            .iter()
            .copied()
            .filter(|&pos| {
                self.board
                    .piece(pos)
                    .map(|p| p.color() == color)
                    .unwrap_or(false)
            })
            .collect();

        for source_pos in player_positions {
            let moves = self.calculate_possible_moves(source_pos);

            for row in 0..self.board.rows {
                for col in 0..self.board.cols {
                    if moves[row][col] {
                        let target_pos = Position::new(row, col);
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

    /// Procura a posição do rei de uma determinada cor.
    fn king(&self, color: Color) -> Option<Position> {
        self.pieces_on_board
            .iter()
            .copied()
            .find(|&pos| {
                if let Some(piece) = self.board.piece(pos) {
                    let display = piece.to_string();
                    let is_king_char = display.contains('♔') || display.contains('♚');
                    is_king_char && piece.color() == color
                } else {
                    false
                }
            })
    }

    fn opponent(&self, color: Color) -> Color {
        match color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    fn next_turn(&mut self) {
        self.turn += 1;
        self.current_player = self.opponent(self.current_player);
    }

    /// Coloca uma nova peça no tabuleiro e registra sua posição em `pieces_on_board`.
    fn place_new_piece(&mut self, pos: ChessPosition, piece: Box<dyn Piece>) {
        let board_pos = pos.to_position();
        self.board
            .place_piece(piece, board_pos)
            .expect("Invariante quebrada: falha ao posicionar nova peça");
        self.pieces_on_board.insert(board_pos);
    }

    /// Configuração inicial padrão de uma partida de xadrez.
    fn initial_setup(&mut self) {
        // Brancas
        self.place_new_piece(ChessPosition { col: 'a', row: 1 }, Box::new(Rook::new(Color::White)));
        self.place_new_piece(ChessPosition { col: 'b', row: 1 }, Box::new(Knight::new(Color::White)));
        self.place_new_piece(ChessPosition { col: 'c', row: 1 }, Box::new(Bishop::new(Color::White)));
        self.place_new_piece(ChessPosition { col: 'd', row: 1 }, Box::new(Queen::new(Color::White)));
        self.place_new_piece(ChessPosition { col: 'e', row: 1 }, Box::new(King::new(Color::White)));
        self.place_new_piece(ChessPosition { col: 'f', row: 1 }, Box::new(Bishop::new(Color::White)));
        self.place_new_piece(ChessPosition { col: 'g', row: 1 }, Box::new(Knight::new(Color::White)));
        self.place_new_piece(ChessPosition { col: 'h', row: 1 }, Box::new(Rook::new(Color::White)));

        for col in b'a'..=b'h' {
            self.place_new_piece(
                ChessPosition {
                    col: col as char,
                    row: 2,
                },
                Box::new(Pawn::new(Color::White)),
            );
        }

        // Pretas
        self.place_new_piece(ChessPosition { col: 'a', row: 8 }, Box::new(Rook::new(Color::Black)));
        self.place_new_piece(ChessPosition { col: 'b', row: 8 }, Box::new(Knight::new(Color::Black)));
        self.place_new_piece(ChessPosition { col: 'c', row: 8 }, Box::new(Bishop::new(Color::Black)));
        self.place_new_piece(ChessPosition { col: 'd', row: 8 }, Box::new(Queen::new(Color::Black)));
        self.place_new_piece(ChessPosition { col: 'e', row: 8 }, Box::new(King::new(Color::Black)));
        self.place_new_piece(ChessPosition { col: 'f', row: 8 }, Box::new(Bishop::new(Color::Black)));
        self.place_new_piece(ChessPosition { col: 'g', row: 8 }, Box::new(Knight::new(Color::Black)));
        self.place_new_piece(ChessPosition { col: 'h', row: 8 }, Box::new(Rook::new(Color::Black)));

        for col in b'a'..=b'h' {
            self.place_new_piece(
                ChessPosition {
                    col: col as char,
                    row: 7,
                },
                Box::new(Pawn::new(Color::Black)),
            );
        }
    }

    /// Lida com o roque (após movimentar o rei duas casas).
    fn handle_castling(&mut self, source: Position, target: Position) {
        let col_diff = (target.col as isize - source.col as isize).abs();
        if col_diff != 2 {
            return;
        }

        // Deve ser um rei
        let moved_piece = self.piece_at(target);
        let display = moved_piece.to_string();
        let is_king = display.contains('♔') || display.contains('♚');
        if !is_king {
            return;
        }

        if target.col > source.col {
            // Roque pequeno
            let rook_source = Position::new(source.row, source.col + 3);
            let rook_target = Position::new(source.row, source.col + 1);

            if let Some(mut rook) = self.board.remove_piece(rook_source) {
                rook.increase_move_count();
                self.board
                    .place_piece(rook, rook_target)
                    .expect("Invariante quebrada: falha ao mover torre no roque pequeno");
                self.pieces_on_board.remove(&rook_source);
                self.pieces_on_board.insert(rook_target);
            }
        } else {
            // Roque grande
            let rook_source = Position::new(source.row, source.col - 4);
            let rook_target = Position::new(source.row, source.col - 1);

            if let Some(mut rook) = self.board.remove_piece(rook_source) {
                rook.increase_move_count();
                self.board
                    .place_piece(rook, rook_target)
                    .expect("Invariante quebrada: falha ao mover torre no roque grande");
                self.pieces_on_board.remove(&rook_source);
                self.pieces_on_board.insert(rook_target);
            }
        }
    }

    /// Desfaz o roque, caso ele tenha ocorrido no movimento desfeito.
    fn undo_castling(&mut self, source: Position, target: Position) {
        let col_diff = (target.col as isize - source.col as isize).abs();
        if col_diff != 2 {
            return;
        }

        let moved_piece = self.piece_at(source);
        let display = moved_piece.to_string();
        let is_king = display.contains('♔') || display.contains('♚');
        if !is_king {
            return;
        }

        if target.col > source.col {
            // Desfaz roque pequeno
            let rook_source = Position::new(source.row, source.col + 1);
            let rook_target = Position::new(source.row, source.col + 3);

            if let Some(mut rook) = self.board.remove_piece(rook_source) {
                rook.decrease_move_count();
                self.board
                    .place_piece(rook, rook_target)
                    .expect("Invariante quebrada: falha ao desfazer roque pequeno");
            }
        } else {
            // Desfaz roque grande
            let rook_source = Position::new(source.row, source.col - 1);
            let rook_target = Position::new(source.row, source.col - 4);

            if let Some(mut rook) = self.board.remove_piece(rook_source) {
                rook.decrease_move_count();
                self.board
                    .place_piece(rook, rook_target)
                    .expect("Invariante quebrada: falha ao desfazer roque grande");
            }
        }
    }
}
