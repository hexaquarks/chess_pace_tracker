use crate::deserialization::*;
use std::collections::HashMap;

// This function is now correctly placed within the test module.
pub fn create_mock_game_json() -> GameJson {
    GameJson {
        clock: Some(Clock {
            increment: Some(10),
            initial: Some(300),
            total_time: Some(600),
        }),
        clocks: Some(vec![300, 290, 280, 270]),
        created_at: Some(1609459200),
        id: Some("game123".to_string()),
        last_move_at: Some(1609459800),
        moves: Some("e2e4 e7e5 d2d4 exd4".to_string()),
        perf: Some("blitz".to_string()),
        players: Some(Players {
            black: Some(PlayerDetail {
                rating: Some(1500),
                rating_diff: Some(10),
                user: Some(User {
                    id: Some("player1".to_string()),
                    name: Some("Alice".to_string()),
                }),
            }),
            white: Some(PlayerDetail {
                rating: Some(1450),
                rating_diff: Some(-10),
                user: Some(User {
                    id: Some("player2".to_string()),
                    name: Some("Bob".to_string()),
                }),
            }),
        }),
        rated: Some(true),
        speed: Some("blitz".to_string()),
        status: Some("finished".to_string()),
        variant: Some("standard".to_string()),
        winner: Some("black".to_string()),
        extra: HashMap::new(),
    }
}