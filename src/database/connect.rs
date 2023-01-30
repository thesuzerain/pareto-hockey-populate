use rusqlite::Connection;

pub const DATABASE_FILE_LOC : &'static str = "pareto-hockey-cache.db";

// Open a connection to SQLite database
pub fn connect() -> rusqlite::Result<()> {
    let conn = Connection::open(DATABASE_FILE_LOC)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS player
        ([id] INTEGER PRIMARY_KEY, 
        [name] TEXT, 
        [gender] VARCHAR(8),
        [position] VARCHAR(16),
        [date_of_birth] DATE, 
        [draft_age] INTEGER, 
        [draft_year] YEAR,
        [round] INTEGER,
        [overall] INTEGER)",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS team
        ([id] INTEGER PRIMARY_KEY, [name] VARCHAR(16))",
        [],
    )?;

    conn.execute(
        "CREATE TABLE [team_season] (
            [id]	INTEGER PRIMARY_KEY,
            [team_id]	INTEGER,
            [season_start_year]	INTEGER,
            [league_slug]	VARCHAR(16),
            [group_name]	VARCHAR(16),
            [games_played]	INTEGER,
            [goals_for]	INTEGER,
            [goals_against]	INTEGER,
            [points]	INTEGER,
            [points_per_game]	FLOAT(3),
            [goal_difference]	INTEGER,
            [wins]	INTEGER,
            [losses]	INTEGER,
            [ties]	INTEGER
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS player_season
        ([id] INTEGER PRIMARY_KEY, 
        [player_id] INTEGER,
        [team_id] INTEGER, 
        [season_start_year] INTEGER,
        [games_played] INTEGER,
        [goals] INTEGER,
        [assists] INTEGER,
        [points] INTEGER,
        [points_per_game] FLOAT(3))",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS league
        ([slug] VARCHAR(8),
        [name] VARCHAR(16), 
        [league_tier] INTEGER)",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS season
        ([season_start_year] YEAR PRIMARY KEY, [season_slug] VARCHAR(16))",
        [],
    )?;

    Ok(())
}
