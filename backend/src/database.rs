use rusqlite::{params, Connection, Result};

pub fn create_database() -> Result<()> {
    let connection = Connection::open("request_timing_data.db")?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS request_logs (
            id INTEGER PRIMARY KEY,
            games_count INTEGER,
            game_mode TEXT,
            user_color TEXT,
            processing_time REAL
        )",
        [],
    )?;
    Ok(())
}

pub fn log_request_data(
    games_count: i32,
    game_mode: &str,
    user_color: &str,
    processing_time: f32,
) -> Result<()> {
    let conn = Connection::open("request_timing_data.db")?;

    conn.execute(
        "INSERT INTO request_logs (games_count, game_mode, user_color, processing_time) VALUES (?1, ?2, ?3, ?4)",
        params![games_count, game_mode, user_color, processing_time],
    )?;
    Ok(())
}
