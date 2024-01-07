use crate::fetch_lichess::{GameFetchWarning, GameInfo};

const MIN_NUMBER_OF_MOVES_IN_GAME: usize = 5;

pub fn process_average_time(games: &Vec<GameInfo>) -> i32 {
    let mut skipped_games: Vec<(usize, GameFetchWarning)> = Vec::new();

    let k = 3;
    for (i, game_info) in games.iter().enumerate() {
        if (game_info.timed_moves.len() < MIN_NUMBER_OF_MOVES_IN_GAME) {
            // skip this game and add it to vector of warnings with warning
            skipped_games.push((i, GameFetchWarning::GameHasNotEnoughMoves));
            continue;
        }
    }
    k
}
