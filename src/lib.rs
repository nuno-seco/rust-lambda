pub mod engine {

    pub mod errors;
    pub mod events;
    mod game;

    use self::{
        errors::{GameError, GameError::GameNotFound},
        events::{ActorEvent, GameEvent},
        game::{Game, GameStatus},
    };
    use game::NUMBER_OF_GUESSES;
    use rand::{prelude::ThreadRng, Rng};

    use std::collections::HashMap;
    use uuid::Uuid;

    pub struct Engine {
        game_repo: HashMap<Uuid, Game>,
        rand_generator: ThreadRng,
    }

    impl Engine {
        pub fn new() -> Self {
            Engine {
                game_repo: HashMap::new(),
                rand_generator: rand::thread_rng(),
            }
        }

        pub fn process(&mut self, event: ActorEvent) -> Result<GameEvent, GameError> {
            match &event {
                ActorEvent::GameInfoRequested { id } => self.get_info(id),
                ActorEvent::GuessSubmitted { id, guess } => self.submit_guess(id, guess),
                ActorEvent::GameRequested => self.create_game(),
            }
        }

        fn get_info(&mut self, id: &Uuid) -> Result<GameEvent, GameError> {
            self.get_game(id)
                .map(|g| GameEvent::GameInfoProvided(g.clone()))
        }

        fn create_game(&mut self) -> Result<GameEvent, GameError> {
            let game = Game::new(
                Uuid::new_v4(),
                self.rand_generator.gen_range(0..10),
                [None, None, None],
            );

            let event = GameEvent::GameCreated(game.clone());
            self.insert_game(game);

            Ok(event)
        }

        fn submit_guess(&mut self, id: &Uuid, guess: &u8) -> Result<GameEvent, GameError> {
            let current_game = self.get_game(id)?;
            let new_guess_list = new_guess_list(current_game.guesses, guess)?;
            let new_status = new_status(new_guess_list, current_game.number);
            let new_game_state = self.update_game(id, new_guess_list, new_status.clone())?;

            match new_status {
                GameStatus::Lost => Ok(GameEvent::GameLost(new_game_state.clone())),
                GameStatus::Ongoing => Ok(GameEvent::GuessEvaluated(new_game_state.clone())),
                GameStatus::Won => Ok(GameEvent::GameWon(new_game_state.clone())),
            }
        }

        fn insert_game(&mut self, game: Game) -> () {
            self.game_repo.insert(game.id, game);
        }

        fn update_game(
            &mut self,
            id: &Uuid,
            new_state: [Option<u8>; NUMBER_OF_GUESSES],
            new_status: GameStatus,
        ) -> Result<&Game, GameError> {
            self.game_repo.entry(*id).and_modify(|game| {
                game.guesses = new_state;
                game.status = new_status
            });

            self.get_game(id)
        }

        fn get_game(&self, id: &Uuid) -> Result<&Game, GameError> {
            self.game_repo.get(id).ok_or_else(|| GameNotFound)
        }
    }

    fn new_status(
        new_guess_list: [Option<u8>; NUMBER_OF_GUESSES],
        selected_number: u8,
    ) -> GameStatus {
        match new_guess_list {
            guesses if guesses.contains(&Some(selected_number)) => GameStatus::Won,
            [Some(_), Some(_), Some(_)] => GameStatus::Lost,
            _ => GameStatus::Ongoing,
        }
    }

    fn new_guess_list(
        current_guesses: [Option<u8>; NUMBER_OF_GUESSES],
        guess: &u8,
    ) -> Result<[Option<u8>; NUMBER_OF_GUESSES], GameError> {
        match current_guesses {
            [None, None, None] => Ok([Some(*guess), None, None]),
            [a @ Some(_), None, None] => Ok([a, Some(*guess), None]),
            [a @ Some(_), b @ Some(_), None] => Ok([a, b, Some(*guess)]),
            [Some(_), Some(_), Some(_)] => Err(GameError::GameFinished),
            _ => Err(GameError::GameInvalid),
        }
    }
}
