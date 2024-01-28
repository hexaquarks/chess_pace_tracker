use crate::api::GameFetchWarning;
use std::collections::HashMap;

pub fn compute_average(times: &[i32]) -> i32 {
    times.iter().sum::<i32>() / times.len() as i32
}

pub fn convert_centiseconds_to_seconds(time: i32) -> f32 {
    (time as f32 / 100 as f32) as f32
}

pub fn generate_dummy_erros_testing(skipped_games: &mut HashMap<usize, GameFetchWarning>) {
    (0..5).for_each(|i| {
        skipped_games
            .entry(i)
            .or_insert(GameFetchWarning::GameHasNotEnoughMoves);
    });
}
