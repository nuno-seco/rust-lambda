use lambda_runtime::{error::HandlerError, lambda, Context};
use log::{self, LevelFilter};
use rust_lambda_chapter_2::events::{ActorEvent, Game, GameEvent};
use simple_logger::SimpleLogger;
use uuid::Uuid;

fn main() -> () {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    lambda!(router);
    // lambda!(|event, context| { router(event, context) })
}

fn router(event: ActorEvent, _: Context) -> Result<GameEvent, HandlerError> {
    let result = match &event {
        ActorEvent::GameInfoRequested { id } => GameEvent::GameInfoProvided(Game {
            number: 47,
            id: *id,
            guesses: [None, None, None],
        }),
        ActorEvent::GuessSubmitted { id, guess } => GameEvent::GuessEvaluated(Game {
            number: 47,
            id: *id,
            guesses: [Some(*guess), None, None],
        }),
        ActorEvent::GameRequested => GameEvent::GameCreated(Game {
            number: 47,
            id: Uuid::new_v4(),
            guesses: [None, None, None],
        }),
    };

    Ok(result)
}
