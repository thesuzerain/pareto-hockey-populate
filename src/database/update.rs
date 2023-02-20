use rusqlite:: Connection;

use crate::database::connect::DATABASE_FILE_LOC;


// TODO: write a better batch update query.
// TODO: move "update" from insert.rs into here


// TODO: game log table should have link to  primary key of player season
// aggregate game log stats into player seasons


pub fn all_updates() -> rusqlite::Result<()> {
    update_calculate_draft_age()?;
    update_aggregate_team_season_records()?;
    update_artificial_team_seasons()?;
    update_game_log_foreign_key()?;
    update_nhl_career_aggregate_stats()?;

    Ok(())
}


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
    conn.execute("DELETE FROM team_season", [])?;
    conn.execute("INSERT INTO team_season(team_id, season_start_year, league_slug, games_played, goals_for, goals_against, points, goal_difference, wins, losses, ties) 
        SELECT team_id, season_start_year, league_slug, SUM(games_played), SUM(goals_for), SUM(goals_against), SUM(points), SUM(goal_difference), SUM(wins), SUM(losses), SUM(ties)
        FROM team_season_group
        GROUP BY team_id, season_start_year, league_slug
    ", [])?;

    // Adds team_season_groupid to 'player_season' and 'team_season_group'
    conn.execute("UPDATE player_season SET team_season_id = NULL",[])?;
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


// Creates artificial 'team_season' records for where player_seasons do not have a corresponding team_season,
// OR records where the team_season does not have games_played field and populate games_played and goals_for artificially
// These records are marked with 'team_season.artificial'
//      0 -> not artificial
//      1 -> partially artificial 
//      2 -> entirely artificial
// They may not be exhaustive: an artificial team season may not have the 'real' number of player_seasons that compose it if such records are missing.
pub fn update_artificial_team_seasons() -> rusqlite::Result<()> {
    let conn = Connection::open(DATABASE_FILE_LOC)?;

    conn.execute("DELETE FROM team_season WHERE artificial = 2", [])?;
    conn.execute("UPDATE team_season SET games_played = NULL, goals_for = NULL, points = NULL, artificial_players_with_games_used = 0 WHERE artificial = 1", [])?;

    // Create new artificial team_seasons (artificial = 2) for player_seasons that do not have corresponding team_seasons
    conn.execute("INSERT INTO team_season (team_id, league_slug, season_start_year, games_played, goals_for, points, artificial, artificial_players_with_games_used) 
        SELECT team_id, league_slug, season_start_year, MAX(player_season.games_played), SUM(player_season.goals), SUM(player_season.points), 2, COUNT(player_season.games_played)
        FROM player_season WHERE team_season_id IS NULL and team_id IS NOT NULL and league_slug IS NOT NULL and season_start_year IS NOT NULL
        GROUP BY team_id, league_slug, season_start_year;
    ", [])?;

    // Update team_seasons (artificial = 1) artificially for team_seasons where the games_played and goals_for are NULL, when
    // there are player_seasons that could have those values.
    conn.execute("UPDATE team_season SET games_played = gp, goals_for = gf, points = pts, artificial_players_with_games_used = apss, artificial = 1 FROM 
    (SELECT MAX(games_played) as gp, SUM(goals) as gf, SUM(points) as pts, COUNT(id) as apss, league_slug, team_id, season_start_year FROM player_season
    GROUP BY league_slug, team_id, season_start_year) as ps
    WHERE team_season.league_slug = ps.league_slug and team_season.team_id = ps.team_id and team_season.season_start_year = ps.season_start_year
    AND (team_season.games_played IS NULL OR team_season.goals_for IS NULL) AND gp IS NOT NULL;

    ", [])?;

    // Adds team_season_groupid to 'player_season' and 'team_season_group'
    // NOTE: this is a duplicate of queries in 'update_aggregate_team_season_records'
    conn.execute("UPDATE player_season SET team_season_id = NULL",[])?;
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
// Updates PlayerSeasonRecord by adding a FK reference to the TeamSeasonGroupRecord table corresponding to the team's stat record for a given player-season-team combo.
// pub fn update_reference_team_season_id() -> rusqlite::Result<()> {

//     let conn = Connection::open(DATABASE_FILE_LOC)?;
//     conn.execute("UPDATE player_season SET team_season_id = (SELECT draft_year - strftime('%Y',date_of_birth))", [])?;

//     Ok(())

// }

pub fn update_game_log_foreign_key() -> rusqlite::Result<()> {

    let conn = Connection::open(DATABASE_FILE_LOC)?;
    conn.execute("UPDATE game_log SET player_season_id = player_season.id FROM player_season 
    WHERE player_season.player_id=game_log.player_id 
        and player_season.team_id=game_log.team_id 
        and player_season.league_slug=game_log.league_slug 
        and player_season.season_start_year=game_log.season_start_year", [])?;
    Ok(())
}


pub fn update_nhl_career_aggregate_stats() -> rusqlite::Result<()> {
    let conn = Connection::open(DATABASE_FILE_LOC)?;
    conn.execute("UPDATE player 
    SET nhl_career_points = 0, 
        nhl_career_games_played = 0;", [])?;
        conn.execute("UPDATE player 
        SET nhl_career_points = ps.pts, 
            nhl_career_games_played = ps.gp 
        FROM (
            SELECT player_season.player_id, SUM(player_season.points) as pts, SUM(player_season.games_played) as gp
            FROM player_season WHERE player_season.league_slug = \"nhl\" GROUP BY player_season.player_id) AS ps
        WHERE 
            player.id = ps.player_id", [])?;
        Ok(())

}