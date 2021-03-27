use std::usize;

use serde_derive::Serialize;
use uuid::Uuid;

pub const NUMBER_OF_GUESSES: usize = 3;

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct Game {
    pub id: Uuid,
    pub guesses: [Option<u8>; 3],
    pub number: u8,
    pub status: GameStatus,
}
#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum GameStatus {
    #[serde(rename = "ongoing")]
    Ongoing,
    #[serde(rename = "won")]
    Won,
    #[serde(rename = "lost")]
    Lost,
}

impl Game {
    pub fn new(id: Uuid, number: u8, guesses: [Option<u8>; NUMBER_OF_GUESSES]) -> Self {
        Game {
            guesses,
            id,
            number,
            status: GameStatus::Ongoing,
        }
    }
}
