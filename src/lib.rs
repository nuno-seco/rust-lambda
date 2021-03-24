pub mod events {
    use serde_derive::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(Deserialize, Debug, PartialEq)]
    #[serde(tag = "kind")]
    pub enum ActorEvent {
        #[serde(rename = "gameRequested")]
        GameRequested,
        #[serde(rename = "gameInfoRequested")]
        GameInfoRequested { id: Uuid },
        #[serde(rename = "guessSubmitted")]
        GuessSubmitted { id: Uuid, guess: u8 },
    }

    #[derive(Serialize, Debug, PartialEq)]
    #[serde(tag = "kind")]
    pub enum GameEvent {
        GameCreated(Game),
        GameInfoProvided(Game),
        GuessEvaluated(Game),
    }
    #[derive(Serialize, Debug, PartialEq)]
    pub struct Game {
        pub id: Uuid,
        pub guesses: [Option<u8>; 3],
    }
}

#[cfg(test)]
mod tests {
    use core::panic;

    use uuid::Uuid;

    use crate::events::ActorEvent;

    #[test]
    fn test_deserialize_session_info_requested() {
        let json =
            "{\"kind\": \"gameInfoRequested\", \"id\": \"5f85938b-b7fe-488f-8dc5-eed7f573d94d\"}";
        let result = serde_json::from_str::<ActorEvent>(json);

        match result {
            Ok(actual) => assert_eq!(
                ActorEvent::GameInfoRequested {
                    id: Uuid::parse_str("5f85938b-b7fe-488f-8dc5-eed7f573d94d").unwrap()
                },
                actual
            ),
            Err(_) => panic!(),
        }
    }

    #[test]
    fn test_deserialize_new_guess_submitted() -> () {
        let json = "{\"kind\": \"guessSubmitted\", \"guess\": 9, \"id\": \"5f85938b-b7fe-488f-8dc5-eed7f573d94d\"}";
        let result = serde_json::from_str::<ActorEvent>(json);
        match result {
            Ok(actual) => assert_eq!(
                ActorEvent::GuessSubmitted {
                    id: Uuid::parse_str("5f85938b-b7fe-488f-8dc5-eed7f573d94d").unwrap(),
                    guess: 9
                },
                actual
            ),
            Err(_) => panic!(),
        }
    }
}
