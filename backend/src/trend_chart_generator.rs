use crate::api::GameFetchWarning;
use crate::game_info_generator::GameInfo;
use crate::util;

use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Debug, PartialEq)]
pub struct TrendChartDatum {
    pub time_differential: f32,
    pub win_status: String,
    pub game_number: i32,
}

pub fn process_trend_chart_data(
    games: &[GameInfo],
    skipped_games: &HashMap<usize, GameFetchWarning>,
    half_time_differentials: Vec<f32>,
) -> Vec<TrendChartDatum> {
    let mut trend_chart_data: Vec<TrendChartDatum> = Vec::new();
    let mut game_number_counter: usize = 1;

    for (i, game_info) in games.iter().enumerate() {
        if skipped_games.contains_key(&i) {
            // The current game has already an internal error.
            // Skip it from the computation.
            continue;
        }

        let trend_chart_datum = TrendChartDatum {
            time_differential: half_time_differentials[game_number_counter - 1],
            win_status: if util::has_user_won_game(game_info) {
                String::from("win")
            } else {
                String::from("loss")
            },
            game_number: game_number_counter as i32,
        };

        trend_chart_data.push(trend_chart_datum);
        game_number_counter += 1;
    }

    trend_chart_data
}
