use rusqlite::Connection;
use rusqlite::NO_PARAMS;

pub const DATABASE_FILE_LOC : &'static str = "pareto-hockey-cache.db";

pub fn connect() -> rusqlite::Result<()> {
    let conn = Connection::open(DATABASE_FILE_LOC)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS player
        ([pid] INTEGER PRIMARY KEY, 
        [player_name] TEXT, 
        [position] VARCHAR(16),
        [date_of_birth] DATE, 
        [draft_age] INTEGER, 
        [draft_year] YEAR,
        [round] INTEGER,
        [overall] INTEGER)",
        NO_PARAMS,
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS team
        ([id] INTEGER PRIMARY_KEY, [name] VARCHAR(16) PRIMARY KEY)",
        NO_PARAMS,
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
        // id, team_id, season_start_year, group, gp, g, a, pts, ppg
        NO_PARAMS,
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS player_season
        ([id] INTEGER, 
        [pid] INTEGER,
        [ts_id] INTEGER, 
        [season_start_year] INTEGER,
        [games_played] INTEGER,
        [goals] INTEGER,
        [assists] INTEGER,
        [points] INTEGER,
        [ppg] FLOAT(3))",
        // id, player_id, team_id, sesaon_start_year, gp, g, a, pts, ppg

        NO_PARAMS,
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS league
        ([slug] VARCHAR(8) PRIMARY KEY,
        [name] VARCHAR(16), 
        [league_tier] INTEGER)",
        NO_PARAMS,
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS season
        ([season_start_year] YEAR PRIMARY KEY, [season_slug] VARCHAR(16))",
        NO_PARAMS,
    )?;

    Ok(())
}
