use serde::{Deserialize, Serialize};
use crate::chess::color::Color;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PieceView {
    pub symbol: String,
    pub color: Color,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GameMessage {
    // Mensagem enviada pelo servidor para informar ao cliente qual cor o servidor atribuiu a ele.
    AssignColor(Color),
    // Solicitação do cliente para entrar na fila/entrar na partida (sem corpo por enquanto)
    Join,
    MakeMove { source: String, target: String },
    GameState { 
        board: Vec<Vec<Option<PieceView>>>,
        turn_color: Color,
        is_check: bool,
        is_check_mate: bool,
        message: String 
    },
    WaitingForOpponent,
    GameEnd { winner: Option<Color> },
    Error(String),
}