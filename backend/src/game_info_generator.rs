use crate::deserialization::GameJson;

#[derive(Clone)]
pub struct TimedMove {
    pub move_key: String,
    pub move_time: i64,
}

pub struct GameInfo {
    pub game_index: usize,
    pub timed_moves: Vec<TimedMove>,
    pub user_color: String,
    pub user_rating: i32,
    pub opponent_rating: i32,
    pub winner_color: String,
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
    let user_rating = get_user_rating(game, &user_color);
    let opponent_color = if user_color == "black" {
        "white"
    } else {
        "black"
    };
    let opponent_rating = get_user_rating(game, opponent_color);
    let game_info = GameInfo {
        game_index: *game_idx,
        timed_moves: generate_timed_moves(game),
        user_color: user_color,
        user_rating: user_rating,
        opponent_rating: opponent_rating,
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

pub fn get_winner_color(game: &GameJson) -> String {
    game.winner.as_ref().unwrap().clone()
}
