use crate::deserialization::*;
use std::collections::HashMap;

pub struct MinimalGameJsonInfoTesting {
    pub clocks: Option<Vec<i64>>,
    pub moves: Option<String>,
    pub black_player_name: Option<String>,
    pub white_player_name: Option<String>,
    pub winner: Option<String>,
}

pub fn get_some_mocked_game_a() -> GameJson {
    create_mock_game_json(MinimalGameJsonInfoTesting {
        clocks: Some(vec![18003, 18003, 17939, 17931, 17899, 17867]),
        moves: Some("e4 c5 Nf3 d6 d4 cxd4".to_string()),
        black_player_name: Some("user".to_string()),
        white_player_name: Some("other_user".to_string()),
        winner: Some("white".to_string()),
    })
} // time diff : (17939 - 17931 = 8)

pub fn get_some_mocked_game_b() -> GameJson {
    create_mock_game_json(MinimalGameJsonInfoTesting {
        clocks: Some(vec![18003, 18003, 17509, 16931, 17201, 16000, 17000]),
        moves: Some("e4 c5 Nf3 d6 d4 cxd4".to_string()),
        black_player_name: Some("other_user".to_string()),
        white_player_name: Some("user".to_string()),
        winner: Some("black".to_string()),
    })
} // time diff : (17509 - 16931 = 578)

pub fn create_mock_game_json(mut info: MinimalGameJsonInfoTesting) -> GameJson {
    GameJson {
        clock: Some(Clock {
            increment: Some(0),
            initial: Some(180),
            total_time: None,
        }),
        clocks: info.clocks.take(),
        created_at: Some(1672371185802),
        id: Some("9kZXlH2K".to_string()),
        last_move_at: Some(1672371338481),
        moves: info.moves.take(),
        perf: Some("blitz".to_string()),
        players: Some(Players {
            black: Some(PlayerDetail {
                rating: Some(2054),
                rating_diff: None,
                user: Some(User {
                    id: Some("player1".to_string()),
                    name: info.black_player_name.take(),
                }),
            }),
            white: Some(PlayerDetail {
                rating: Some(2000),
                rating_diff: None,
                user: Some(User {
                    id: Some("player2".to_string()),
                    name: info.white_player_name.take(),
                }),
            }),
        }),
        rated: Some(true),
        speed: Some("blitz".to_string()),
        status: Some("mate".to_string()),
        variant: Some("standard".to_string()),
        winner: info.winner.take(),
        extra: HashMap::new(),
    }
}
