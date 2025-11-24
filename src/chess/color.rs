use serde::{Deserialize, Serialize}; // <-- Importante: Importar isso

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)] // <-- Adicionar Serialize e Deserialize aqui
pub enum Color {
    Black,
    White,
}