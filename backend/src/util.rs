use actix::Addr;
use std::collections::HashMap;

use crate::games_info_generator::GameInfo;
use crate::service_intermediary::GameFetchWarning;
use crate::websocket::{WebSocketSession, WebSocketTextMessage};

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
        panic!(); // Why did I add this lol?
    }

    if is_game_draw(game) {
        // No one flagged anyone
        return None;
    }

    // TODO extract var and check what other types of game status the API can return.
    if game.game_status == "outoftime" {
        return Some(has_user_won_game(game));
    }

    None
}

pub fn send_websocket_message(
    websocket_addr_lock: &Addr<WebSocketSession>,
    game_idx: usize,
    games_count: &i32,
) {
    let message = WebSocketTextMessage(format!("Game {}/{}", game_idx + 1, games_count));

    websocket_addr_lock.do_send(message);
}
