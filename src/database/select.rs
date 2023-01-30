use rusqlite::{Connection};
use crate::database::connect::DATABASE_FILE_LOC;

// Get list of all Player ids stored in the database
pub fn select_player_ids() -> rusqlite::Result<Vec<u32>>{

    let conn = Connection::open(DATABASE_FILE_LOC)?;

    let mut res = conn.prepare("SELECT id FROM player")?;

    let player_ids = res.query_map([],
         |row| {
            row.get(0)
        }
        )?.filter_map(|p| p.ok()).collect();
    Ok(player_ids)
}
