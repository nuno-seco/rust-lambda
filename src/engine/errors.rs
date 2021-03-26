#[derive(Debug)]
pub enum GameError {
    GameNotFound,
    GameFinished,
    GameInvalid,
}

impl ToString for GameError {
    fn to_string(&self) -> String {
        match *self {
            GameError::GameNotFound => "Game Not Found".to_owned(),
            GameError::GameFinished => "Game Already Finished".to_owned(),
            GameError::GameInvalid => "Game Invalid".to_owned(),
        }
    }
}
