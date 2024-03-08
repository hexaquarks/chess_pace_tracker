use std::collections::HashMap;

use crate::games_info_generator::{GameInfo, TimedMove};
use crate::service_intermediary::GameFetchWarning;

pub fn compute_average(times: &[f32]) -> f32 {
    (times.iter().sum::<f32>() / times.len() as f32) as f32
}

pub fn convert_centiseconds_to_seconds(time: i32) -> f32 {
    (time as f32 / 100.0) as f32
}

pub fn generate_dummy_erros_testing(skipped_games: &mut HashMap<usize, GameFetchWarning>) {
    (0..6).for_each(|i| {
        skipped_games.entry(i).or_insert(if i % 2 == 0 {
            GameFetchWarning::GameHasNotEnoughMoves
        } else {
            GameFetchWarning::InternalErrorOccuredWhileProcessingAGame
        });
    });
}

pub fn is_game_draw(game: &GameInfo) -> bool {
    game.winner_color.is_none()
}

pub fn has_user_won_game(game: &GameInfo) -> bool {
    if game.winner_color.is_some() {
        return game.user_color == *game.winner_color.as_ref().unwrap();
    }
    false
}

pub fn get_game_flagging_information(game: &GameInfo) -> Option<bool> {
    let times = game
        .timed_moves
        .iter()
        .map(|time_move| time_move.move_time)
        .collect::<Vec<i64>>();

    if times.len() < 3 {
        panic!();
    }

    let mut white_time = times[0];
    let mut black_time = times[1];

    for (index, time_spent) in times[2..].iter().enumerate() {
        if index % 2 == 0 {
            white_time -= time_spent;
            if white_time <= 0 {
                // Black flagged white
                return Some(game.user_color == "black".to_string());
            }
        } else {
            black_time -= time_spent;
            if black_time <= 0 {
                // White flagged black
                return Some(game.user_color == "white".to_string());
            }
        }
    }

    None // No one flagged anyone
}
