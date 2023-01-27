use rusqlite::Connection;

pub const DATABASE_FILE_LOC : &'static str = "pareto-hockey-cache.db";

// Open a connection to SQLite database
pub fn connect() -> rusqlite::Result<()> {
    let conn = Connection::open(DATABASE_FILE_LOC)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS player
        ([id] INTEGER, 
        [name] TEXT, 
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
        ([id] INTEGER PRIMARY_KEY, [name] VARCHAR(16) PRIMARY KEY)",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS team_season
        
        ([id] INTEGER,
            [team_id] INTEGER,
            [season_start_year] INTEGER,
            [group] VARCHAR(16),
            [games_played] INTEGER,
            [goals] INTEGER,
            [assists] INTEGER,
            [points] INTEGER,
            [ppg] FLOAT(3))",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS player_season
        ([id] INTEGER, 
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
