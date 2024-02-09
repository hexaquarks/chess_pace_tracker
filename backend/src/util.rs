use crate::api::GameFetchWarning;
use crate::game_info_generator::GameInfo;
use std::collections::HashMap;

pub fn compute_average(times: &[i32]) -> f32 {
    (times.iter().sum::<i32>() as f32 / times.len() as f32) as f32
}

pub fn convert_centiseconds_to_seconds(time: f32) -> f32 {
    (time / 100 as f32) as f32
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
