pub mod engine {
    pub mod errors;
    pub mod events;
    mod game;

    use self::{
        errors::{GameError, GameError::GameNotFound},
        events::{ActorEvent, GameEvent},
        game::Game,
    };
    use rand::{prelude::ThreadRng, Rng};

    use std::collections::HashMap;
    use uuid::Uuid;

    pub struct Engine {
        data_store: HashMap<Uuid, Game>,
        rand_generator: ThreadRng,
    }

    impl Engine {
        pub fn new() -> Self {
            Engine {
                data_store: HashMap::new(),
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

        fn create_game(&mut self) -> Result<GameEvent, GameError> {
            let game = Game::new(
                Uuid::new_v4(),
                self.rand_generator.gen_range(0..10),
                [None, None, None],
            );

            self.data_store.insert(game.id, game.clone());

            Ok(GameEvent::GameCreated(game))
        }

        fn submit_guess(&mut self, id: &Uuid, guess: &u8) -> Result<GameEvent, GameError> {
            let new_guess_list = self.new_guess_list(id, guess)?;

            let new_game_state = self.persist(id, new_guess_list);

            match new_game_state {
                Game {
                    guesses, number, ..
                } if guesses.contains(&Some(*number)) => {
                    Ok(GameEvent::GameWon(new_game_state.clone()))
                }
                Game {
                    guesses: [Some(_), Some(_), Some(_)],
                    ..
                } => Ok(GameEvent::GameLost(new_game_state.clone())),
                _ => Ok(GameEvent::GuessEvaluated(new_game_state.clone())),
            }
        }

        fn persist(&mut self, id: &Uuid, new_state: [Option<u8>; 3]) -> &Game {
            self.data_store
                .entry(*id)
                .and_modify(|game| game.guesses = new_state);

            self.data_store.get(id).unwrap()
        }

        fn new_guess_list(&mut self, id: &Uuid, guess: &u8) -> Result<[Option<u8>; 3], GameError> {
            fn next_state(game: &Game, guess: u8) -> Result<[Option<u8>; 3], GameError> {
                match game.guesses {
                    [None, None, None] => Ok([Some(guess), None, None]),
                    [a @ Some(_), None, None] => Ok([a, Some(guess), None]),
                    [a @ Some(_), b @ Some(_), None] => Ok([a, b, Some(guess)]),
                    [Some(_), Some(_), Some(_)] => Err(GameError::GameFinished),
                    _ => Err(GameError::GameInvalid),
                }
            }

            let new_state = self
                .data_store
                .get(id)
                .ok_or_else(|| GameNotFound)
                .and_then(|g| next_state(g, *guess))?;

            Ok(new_state)
        }

        fn get_info(&mut self, id: &Uuid) -> Result<GameEvent, GameError> {
            self.data_store
                .get(id)
                .ok_or_else(|| GameNotFound)
                .map(|g| GameEvent::GameInfoProvided(g.clone()))
        }
    }
}
