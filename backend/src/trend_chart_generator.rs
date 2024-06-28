use crate::games_info_generator::GameInfo;
use crate::service_intermediary::GameFetchWarning;
use crate::util;

use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Debug, PartialEq)]
pub struct TrendChartDatum {
    pub time_differential: f32,
    pub win_status: String,
    pub game_number: i32,
}

fn get_displayable_game_number(game_number: usize, nb_skipped_games: usize) -> i32 {
    (game_number + 1 + nb_skipped_games) as i32
}

fn get_displayable_game_win_status(game_info: &GameInfo) -> String {
    if util::has_user_won_game(game_info) {
        String::from("win")
    } else {
        String::from("loss")
    }
}

pub fn generate(
    games: &[GameInfo],
    skipped_games: &HashMap<usize, GameFetchWarning>,
    half_time_differentials: Vec<f32>,
) -> Vec<TrendChartDatum> {
    let mut trend_chart_data: Vec<TrendChartDatum> = Vec::new();
    let mut game_number_counter: usize = 0;
    let mut nb_skipped_games: usize = 0;

    for (i, game_info) in games.iter().enumerate() {
        if skipped_games.contains_key(&i) {
            // The current game has already an internal error.
            // Skip it from the computation.
            nb_skipped_games += 1;
            continue;
        }

        let trend_chart_datum = TrendChartDatum {
            time_differential: half_time_differentials[game_number_counter],
            win_status: get_displayable_game_win_status(game_info),
            game_number: get_displayable_game_number(game_number_counter, nb_skipped_games),
        };

        trend_chart_data.push(trend_chart_datum);
        game_number_counter += 1;
    }

    trend_chart_data
}
