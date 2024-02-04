use crate::api::GameFetchWarning;
use crate::game_info_generator::{GameInfo, TimedMove};
use crate::util;
use std::collections::HashMap;

const MIN_NUMBER_OF_MOVES_IN_GAME: usize = 5;

fn is_game_draw(game: &GameInfo) -> bool {
    game.winner_color.is_none()
}

fn has_user_won_game(game: &GameInfo) -> bool {
    if game.winner_color.is_some() {
        return game.user_color == *game.winner_color.as_ref().unwrap();
    }
    false
}

/// Heuristics:
/// Check unit tests at bottom of file
fn get_half_moves(
    timed_moves: &[TimedMove],
    midpoint: usize,
    is_moves_even: bool,
    is_user_first_mover: bool,
) -> (TimedMove, TimedMove) {
    let (user_index, opponent_index) = if is_moves_even {
        (midpoint - 1, midpoint)
    } else {
        (midpoint, midpoint - 1)
    };

    let user_move = timed_moves[user_index].clone();
    let opponent_move = timed_moves[opponent_index].clone();

    // Standardize order of return user first and opponent second.
    if (is_user_first_mover && is_moves_even) || (!is_user_first_mover && !is_moves_even) {
        (opponent_move, user_move)
    } else {
        (user_move, opponent_move)
    }
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
        if game_info.timed_moves.len() < MIN_NUMBER_OF_MOVES_IN_GAME {
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

pub fn process_win_rate(
    games: &[GameInfo],
    skipped_games: &HashMap<usize, GameFetchWarning>,
) -> f32 {
    let mut n_games_considered = games.len();
    let mut n_wins = 0;

    for (i, game_info) in games.iter().enumerate() {
        if skipped_games.contains_key(&i) || is_game_draw(game_info) {
            // The current game has already an internal error.
            // Skip it from the computation.
            n_games_considered -= 1;
            continue;
        }

        if has_user_won_game(game_info) {
            n_wins += 1;
        }
    }

    if n_wins == 0 {
        return 0.0;
    }

    n_wins as f32 / n_games_considered as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_move_sequence_test(n_moves: usize) -> Vec<TimedMove> {
        let mut result = Vec::new();
        for i in 0..n_moves {
            let character = (b'A' + i as u8 % 26) as char;
            let number = i + 1;
            result.push(TimedMove {
                move_key: character.to_string(),
                move_time: number as i64,
            });
        }
        result
    }

    #[test]
    fn test_get_half_moves() {
        // A B C D
        // even moves, user goes first
        {
            let even_number_of_moves_user_first = make_move_sequence_test(4);
            let half_move = get_half_moves(
                &even_number_of_moves_user_first,
                even_number_of_moves_user_first.len() / 2,
                true,
                true,
            );

            assert_eq!(half_move.0.move_key, "C");
            assert_eq!(half_move.1.move_key, "B");
        }

        // even moves, opponent goes first
        {
            let even_number_of_moves_opponent_first = make_move_sequence_test(4);
            let half_move = get_half_moves(
                &even_number_of_moves_opponent_first,
                even_number_of_moves_opponent_first.len() / 2,
                true,
                false,
            );

            assert_eq!(half_move.0.move_key, "B");
            assert_eq!(half_move.1.move_key, "C");
        }

        // A B C D E
        // odd moves, user goes first
        {
            let odd_number_of_moves_user_first = make_move_sequence_test(4);
            let half_move = get_half_moves(
                &odd_number_of_moves_user_first,
                odd_number_of_moves_user_first.len() / 2,
                false,
                true,
            );

            assert_eq!(half_move.0.move_key, "C");
            assert_eq!(half_move.1.move_key, "B");
        }

        // odd moves, opponent goes first
        {
            let odd_number_of_moves_opponent_first = make_move_sequence_test(4);
            let half_move = get_half_moves(
                &odd_number_of_moves_opponent_first,
                odd_number_of_moves_opponent_first.len() / 2,
                false,
                false,
            );

            assert_eq!(half_move.0.move_key, "B");
            assert_eq!(half_move.1.move_key, "C");
        }
    }
}
