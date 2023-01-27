use std::cmp;
use rusqlite::{params, Connection, ToSql};

use crate::models::*;
use crate::database::models::*;
use crate::database::connect::DATABASE_FILE_LOC;

// Inserts Vec of DraftSelection information into local database 
// The player records must already exist: this adds the DRAFT aspect of the  PlayerRecrod to existing records in the database.
// Converts: EP 'DraftSelection' to Pareto 'PlayerRecord'
pub fn update_player_partial_draftselection(draft_selections: Vec<draft_selection::DraftSelection>) -> rusqlite::Result<()>{

    // Converts 'DraftSelection's to 'PlayerRecord's WHERE POSSIBLE.
    // 'DraftSelection' structs that do not contain a player id are dropped
    let conn = Connection::open(DATABASE_FILE_LOC)?;
    let players = draft_selections.into_iter().filter_map(|d| player_record::PlayerRecord::from_partial_draftselection(d));

    // Updates existing 'player' table with draft information by playerid
    // TODO
    for p in players {
        conn.execute(
            "UPDATE player SET draft_age = ?2, draft_year = ?3, draft_year = ?4, draft_year = ?5 WHERE id = ?1",
            params![p.id, p.draft_age, p.draft_year, p.draft_round, p.draft_overall],
        )?;
    }
    Ok(())
}

// TODO: write function to batch update query so we can do this faster.
// Mimic like so:
/*
   UPDATE player 
   SET overall = CASE id 
   WHEN 1 THEN 222
   WHEN 2 THEN 1
   ELSE overall END WHERE id IN(1,2)
*/
