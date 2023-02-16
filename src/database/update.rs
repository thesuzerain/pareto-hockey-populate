use rusqlite:: Connection;

use crate::database::connect::DATABASE_FILE_LOC;


// TODO: write a better batch update query.
// TODO: move "update" from insert.rs into here


// TODO: game log table should have link to  primary key of player season
// aggregate game log stats into player seasons


// Updates database by calculating draft age- from data already available in the database.
// Does not add rows, but uses existing 'date_of_birth' and 'draft_year' rows. (So both Player and DraftSelection should already be inserted before running this)
pub fn update_calculate_draft_age() -> rusqlite::Result<()> {

    let conn = Connection::open(DATABASE_FILE_LOC)?;
    conn.execute("UPDATE player SET draft_age = (SELECT draft_year - strftime('%Y',date_of_birth))", [])?;

    Ok(())

}

// Coalesques 'team_season_group' data into aggregate 'team_season_group stats, combining the groups together
// Sets 'team_season_group ids to be accessible in both 'team_season_group' and 'player_season'
// TODO: rename this. Also, should this be in 'update'?
pub fn update_aggregate_team_season_records() -> rusqlite::Result<()> {
    let conn = Connection::open(DATABASE_FILE_LOC)?;

    // Creates aggregate 'team_season table (aggregate over 'team_season_group')
    conn.execute("INSERT INTO team_season(team_id, season_start_year, league_slug, games_played, goals_for, goals_against, points, goal_difference, wins, losses, ties) 
        SELECT team_id, season_start_year, league_slug, SUM(games_played), SUM(goals_for), SUM(goals_against), SUM(points), SUM(goal_difference), SUM(wins), SUM(losses), SUM(ties)
        FROM team_season_group
        GROUP BY team_id, season_start_year, league_slug
    ", [])?;

    // Adds team_season_groupid to 'player_season' and 'team_season_group'
    conn.execute("UPDATE player_season SET team_season_id = team_season.id FROM team_season 
    WHERE player_season.team_id=team_season.team_id 
        and player_season.league_slug=team_season.league_slug 
        and player_season.season_start_year=team_season.season_start_year
    ", [])?;
    conn.execute("UPDATE team_season_group SET team_season_id = team_season.id FROM team_season 
    WHERE team_season_group.team_id=team_season.team_id 
        and team_season_group.league_slug=team_season.league_slug 
        and team_season_group.season_start_year=team_season.season_start_year
    ", [])?;


    Ok(())
}


// Creates artificial 'team_season' records for where player_seasons do not have a corresponding team_season
// These records are marked with 'team_season.artificial = 1'
// They may not be exhaustive: an artificial team season may not have the 'real' number of player_seasons that compose it if such records are missing.
pub fn update_missing_team_seasons() -> rusqlite::Result<()> {
    let conn = Connection::open(DATABASE_FILE_LOC)?;

    conn.execute("DELETE FROM team_season WHERE artificial = 1", [])?;

    conn.execute("INSERT INTO team_season (team_id, league_slug, season_start_year, games_played, goals_for, points, artificial) 
        SELECT team_id, league_slug, season_start_year, SUM(player_season.games_played), SUM(player_season.goals), SUM(player_season.points), 1    FROM player_season WHERE team_season_id IS NULL and team_id IS NOT NULL and league_slug IS NOT NULL and season_start_year IS NOT NULL GROUP BY team_id, league_slug, season_start_year
    ", [])?;

    Ok(())
}
// Updates PlayerSeasonRecord by adding a FK reference to the TeamSeasonGroupRecord table corresponding to the team's stat record for a given player-season-team combo.
// pub fn update_reference_team_season_id() -> rusqlite::Result<()> {

//     let conn = Connection::open(DATABASE_FILE_LOC)?;
//     conn.execute("UPDATE player_season SET team_season_id = (SELECT draft_year - strftime('%Y',date_of_birth))", [])?;

//     Ok(())

// }
