use serde_derive::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug, PartialEq)]
pub struct Game {
    pub id: Uuid,
    pub guesses: [Option<u8>; 3],
    pub number: u8,
}

impl Game {
    pub fn new(id: Uuid, number: u8, guesses: [Option<u8>; 3]) -> Self {
        Game {
            guesses,
            id,
            number,
        }
    }
}

impl Clone for Game {
    fn clone(&self) -> Self {
        Game { ..*self }
    }
}
