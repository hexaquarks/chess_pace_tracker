use crate::api::GameFetchWarning;
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
