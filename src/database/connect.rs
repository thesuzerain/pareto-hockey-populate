use rusqlite::Connection;

pub const DATABASE_FILE_LOC : &'static str = "pareto-hockey-cache.db";

// Ensure all tables are created and exist
pub fn establish_schema() -> rusqlite::Result<()> {
    let conn = Connection::open(DATABASE_FILE_LOC)?;

    // TODO: if switch to postgre from sqlite, can remove 'not null' from primary key

    conn.execute(
        "CREATE TABLE IF NOT EXISTS player
        ([id] INTEGER NOT NULL, 
        [name] TEXT, 
        [gender] VARCHAR(8),
        [position] VARCHAR(16),
        [date_of_birth] DATE, 
        [draft_age] INTEGER, 
        [draft_year] YEAR,
        [round] INTEGER,
        [overall] INTEGER,
        PRIMARY KEY(id))",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS team
        ([id] INTEGER NOT NULL, [name] VARCHAR(16), [logo_url] TEXT,
        PRIMARY KEY(id))",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS [team_season] (
            [id] INTEGER NOT NULL ,
            [team_id]	INTEGER,
            [season_start_year]	INTEGER,
            [league_slug]	VARCHAR(16),
            [games_played]	INTEGER,
            [goals_for]	INTEGER,
            [goals_against]	INTEGER,
            [points]	INTEGER,
            [goal_difference]	INTEGER,
            [wins]	INTEGER,
            [losses]	INTEGER,
            [ties]	INTEGER,
            PRIMARY KEY(id)
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS [team_season_group] (
            [id]	INTEGER NOT NULL,
            [team_season_id] INTEGER,
            [group_name]	VARCHAR(16),
            [team_id]	INTEGER,
            [season_start_year]	INTEGER,
            [league_slug]	VARCHAR(16),
            [games_played]	INTEGER,
            [goals_for]	INTEGER,
            [goals_against]	INTEGER,
            [points]	INTEGER,
            [goal_difference]	INTEGER,
            [wins]	INTEGER,
            [losses]	INTEGER,
            [ties]	INTEGER,
            PRIMARY KEY(id)
        )", 
        [],
    )?;


    conn.execute(
        "CREATE TABLE IF NOT EXISTS player_season
        ([id] INTEGER NOT NULL, 
        [player_id] INTEGER,
        [team_id] INTEGER, 
        [league_slug] VARCHAR(16),
        [season_start_year] INTEGER,
        [games_played] INTEGER,
        [goals] INTEGER,
        [assists] INTEGER,
        [points] INTEGER,
        [team_season_id] INTEGER,
        PRIMARY KEY (id))",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS league
        ([slug] VARCHAR(8),
        [name] VARCHAR(16), 
        [league_tier] INTEGER, [logo_url] TEXT,
        PRIMARY KEY(slug))",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS season
        ([season_start_year] YEAR PRIMARY KEY, [season_slug] VARCHAR(16))",
        [],
    )?;

    Ok(())
}
