use rusqlite::Connection;

use super::connect;

pub fn erase() -> rusqlite::Result<()> {
    erase_league()?;
    erase_team()?;
    erase_team_season()?;
    erase_player()?;
    erase_player_season()?;
    Ok(())
}

pub fn erase_league() -> rusqlite::Result<()> {
    let conn = Connection::open(connect::DATABASE_FILE_LOC)?;
    conn.execute("DROP TABLE IF EXISTS league", [])?;
    Ok(())
}

pub fn erase_team() -> rusqlite::Result<()> {
    let conn = Connection::open(connect::DATABASE_FILE_LOC)?;
    conn.execute("DROP TABLE IF EXISTS team", [])?;
    Ok(())
}

pub fn erase_team_season() -> rusqlite::Result<()> {
    let conn = Connection::open(connect::DATABASE_FILE_LOC)?;
    conn.execute("DROP TABLE IF EXISTS team_season", [])?;
    Ok(())
}

pub fn erase_player() -> rusqlite::Result<()> {
    let conn = Connection::open(connect::DATABASE_FILE_LOC)?;
    conn.execute("DROP TABLE IF EXISTS player", [])?;
    Ok(())
}

pub fn erase_player_season() -> rusqlite::Result<()> {
    let conn = Connection::open(connect::DATABASE_FILE_LOC)?;
    conn.execute("DROP TABLE IF EXISTS player_season", [])?;
    Ok(())
}

pub fn erase_season() -> rusqlite::Result<()> {
    let conn = Connection::open(connect::DATABASE_FILE_LOC)?;
    conn.execute("DROP TABLE IF EXISTS season", [])?;
    Ok(())
}