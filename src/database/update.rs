use rusqlite:: Connection;

use crate::database::connect::DATABASE_FILE_LOC;


// TODO: write a better batch update query.
// TODO: move "update" from insert.rs into here


// Updates database by calculating draft age- from data already available in the database.
// Does not add rows, but uses existing 'date_of_birth' and 'draft_year' rows. (So both Player and DraftSelection should already be inserted before running this)
pub fn update_calculate_draft_age() -> rusqlite::Result<()> {

    let conn = Connection::open(DATABASE_FILE_LOC)?;
    conn.execute("UPDATE player SET draft_age = (SELECT draft_year - strftime('%Y',date_of_birth))", [])?;

    Ok(())

}
