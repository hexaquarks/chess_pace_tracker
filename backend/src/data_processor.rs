use crate::api::GameFetchWarning;
use crate::game_info_generator::{GameInfo, TimedMove};
use crate::util;
use std::collections::HashMap;

const MIN_NUMBER_OF_MOVES_IN_GAME: usize = 5;

fn get_half_moves(
    timed_moves: &[TimedMove],
    middle_cut_idx: usize,
    is_n_moves_even: bool,
    is_user_the_first_to_move: bool,
) -> (TimedMove, TimedMove) {
    let offset = if is_n_moves_even == is_user_the_first_to_move {
        0
    } else {
        1
    };
    let user_half_move = timed_moves[middle_cut_idx + offset].clone();
    let opponent_half_move = timed_moves[middle_cut_idx + 1 - offset].clone();

    (user_half_move, opponent_half_move)
}

pub fn compute_curr_game_time_differential(game: &GameInfo) -> i32 {
    let timed_moves: &[TimedMove] = game.timed_moves.as_ref();

    let middle_cut_idx: usize = timed_moves.len() / 2;
    let is_user_the_first_to_move = game.user_color == timed_moves[0].move_key;
    let is_n_moves_even = timed_moves.len() % 2 == 0;

    let (user_half_move, opponent_half_move) = get_half_moves(
        &timed_moves,
        middle_cut_idx,
        is_n_moves_even,
        is_user_the_first_to_move,
    );

    (user_half_move.move_time - opponent_half_move.move_time) as i32
}

pub fn process_average_time(
    games: &[GameInfo],
    skipped_games: &mut HashMap<usize, GameFetchWarning>,
) -> Option<f32> {
    let mut half_time_differentials = Vec::new();
    for (i, game_info) in games.iter().enumerate() {
        if skipped_games.contains_key(&i) {
            // The current game has already an internal error.
            // Skip it from the computation.
            continue;
        }
        if game_info.timed_moves.len() > MIN_NUMBER_OF_MOVES_IN_GAME {
            // skip this game and add it to vector of warnings with warning
            skipped_games
                .entry(i)
                .or_insert(GameFetchWarning::GameHasNotEnoughMoves);
            continue;
        }

        let curr_game_time_differential = compute_curr_game_time_differential(game_info);
        half_time_differentials.push(curr_game_time_differential);
    }

    if half_time_differentials.len() == 0 {
        // NO games were kept in the computation. The time average is undefined
        return None;
    }

    let average_half_time_differentials = util::compute_average(&half_time_differentials);
    Some(util::convert_centiseconds_to_seconds(
        average_half_time_differentials,
    ))
}
