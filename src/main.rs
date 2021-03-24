use lambda_runtime::{error::HandlerError, lambda, Context};
use log::{self, LevelFilter};
use rust_lambda_chapter_2::events::{ActorEvent, GameEvent, Session};
use simple_logger::SimpleLogger;
use uuid::Uuid;

fn main() -> () {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    lambda!(router);
}

fn router(event: ActorEvent, _: Context) -> Result<GameEvent, HandlerError> {
    let result = match &event {
        ActorEvent::SessionInfoRequested { id } => GameEvent::SessionInfoProvided(Session {
            id: *id,
            guesses: [None, None, None],
        }),
        ActorEvent::GuessSubmitted { id, guess } => GameEvent::GuessEvaluated(Session {
            id: *id,
            guesses: [Some(*guess), None, None],
        }),
        ActorEvent::SessionRequested => GameEvent::SessionCreated(Session {
            id: Uuid::new_v4(),
            guesses: [None, None, None],
        }),
    };

    Ok(result)
}
