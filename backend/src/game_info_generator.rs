use crate::deserialization::*;
use crate::unit_test_util::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TimedMove {
    pub move_key: String,
    pub move_time: i64,
}

pub struct GameInfo {
    pub game_index: usize,
    pub timed_moves: Vec<TimedMove>,
    pub user_color: String,
    pub winner_color: Option<String>,
}

pub fn generate_timed_moves(game: &GameJson) -> Vec<TimedMove> {
    let mut timed_moves: Vec<TimedMove> = Vec::new();

    let moves: Vec<String> = game
        .moves
        .as_ref()
        .unwrap()
        .split(" ")
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    let mut clocks: Vec<i64> = game
        .clocks
        .as_ref()
        .unwrap()
        .iter()
        .cloned()
        .collect::<Vec<_>>();

    if clocks.len() > moves.len() {
        // if the last move in the game was a checkmate, the last
        // clock stamp does not register.
        clocks.truncate(clocks.len() - 1);
    }

    for (i, x) in moves.iter().cloned().enumerate() {
        timed_moves.push(TimedMove {
            move_key: x,
            move_time: clocks[i],
        });
    }
    timed_moves
}

pub fn generate_game_info_struct(
    game: &GameJson,
    game_idx: &usize,
    user_name: &String,
) -> GameInfo {
    let user_color = get_user_color(game, user_name);
    //let user_rating = get_user_rating(game, &user_color);
    let opponent_color = if user_color == "black" {
        "white"
    } else {
        "black"
    };
    //let opponent_rating = get_user_rating(game, opponent_color);
    let game_info = GameInfo {
        game_index: *game_idx,
        timed_moves: generate_timed_moves(game),
        user_color: user_color,
        winner_color: get_winner_color(game),
    };
    game_info
}

pub fn get_user_color(game: &GameJson, user_name: &str) -> String {
    if let Some(players) = game.players.as_ref() {
        if let Some(black_player) = players.black.as_ref() {
            if let Some(user) = black_player.user.as_ref() {
                if user.name.as_ref() == Some(&user_name.to_string()) {
                    return "black".to_string();
                }
            }
        }
    }
    "white".to_string()
}

pub fn get_user_rating(game: &GameJson, user_color: &str) -> i32 {
    let players = match game.players.as_ref() {
        Some(players) => players,
        None => return 0,
    };

    let player_detail = match user_color {
        "black" => &players.black,
        _ => &players.white,
    };

    player_detail.as_ref().unwrap().rating.unwrap_or(0)
}

pub fn get_winner_color(game: &GameJson) -> Option<String> {
    match game.winner {
        Some(_) => Some(game.winner.as_ref().unwrap().clone()),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_timed_moves() {
        let game_json = get_some_mocked_game_a();
        let res: Vec<TimedMove> = generate_timed_moves(&game_json);

        assert_eq!(res.len(), 6);
        assert_eq!(res[0].move_key, "e4");
        assert_eq!(res[0].move_time, 18003);
        assert_eq!(res[1].move_key, "c5");
        assert_eq!(res[3].move_time, 17931);
    }

    #[test]
    fn test_generate_game_info_struct() {
        // Test with mockes game A
        {
            let game_json = get_some_mocked_game_a();
            let res = generate_game_info_struct(&game_json, &0, &"user".to_string());

            assert_eq!(res.game_index, 0);
            assert_eq!(res.timed_moves[0], generate_timed_moves(&game_json)[0]);
            assert_eq!(res.user_color, "black");
            assert_eq!(res.winner_color, Some("white".to_string()));
        }

        // Test with mocked game B
        {
            let game_json = get_some_mocked_game_b();
            let res = generate_game_info_struct(&game_json, &1, &"user".to_string());

            assert_eq!(res.game_index, 1);
            assert_eq!(res.timed_moves[3], generate_timed_moves(&game_json)[3]); // any idx
            assert_eq!(res.user_color, "white");
            assert_eq!(res.winner_color, Some("black".to_string()));
        }
    }
}
