use lambda_runtime::{error::HandlerError, lambda};
use log::{self, LevelFilter};
use rust_lambda_chapter_2::engine::{events::GameEvent, Engine};
use simple_logger::SimpleLogger;

fn main() -> () {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    let mut engine = Engine::new();
    lambda!(|actor_event, _| -> Result<GameEvent, HandlerError> {
        log::info!("Received event : {:?}", actor_event);
        let game_event = engine
            .process(actor_event)
            .map_err(|game_error| game_error.to_string().as_str().into());
        log::info!("Generating : {:?}", game_event);

        game_event
    })
}
