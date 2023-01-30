use std::cmp;
use rusqlite::{params, Connection, ToSql};
use crate::models::*;
use crate::database::models::*;
use crate::database::connect::DATABASE_FILE_LOC;
use crate::batch_params;

use super::BATCH_MAX_SIZE;


// Inserts Vec of Player information into local database 
// Converts: EP 'Player' to Pareto 'PlayerRecord'
pub fn insert_player_partial_player(players: Vec<player::Player>) -> rusqlite::Result<()>{

    // Converts 'Player's to 'PlayerRecord's
    let players : Vec<player_record::PlayerRecord> = players.into_iter().map(|p| player_record::PlayerRecord::from_partial_player(p)).collect();

    // Creates parameter list from PlayerRecords for SQL insertion
    let mut params = Vec::new();
    for p in players.iter() {
        params.push(batch_params!(p.id, p.name, p.gender,  p.position, p.date_of_birth))
    }

    // Insert parameters in batches
    batch_insert_query("INSERT INTO player(id, name, gender, position, date_of_birth) VALUES ", params)?;
    Ok(())
}

// Inserts Vec of DraftSelection information into local database 
// Converts: EP 'DraftSelection' to Pareto 'PlayerRecord'
pub fn insert_player_partial_draftselection(draft_selections: Vec<draft_selection::DraftSelection>) -> rusqlite::Result<()>{

    // Converts 'DraftSelection's to 'PlayerRecord's WHERE POSSIBLE.
    // 'DraftSelection' structs that do not contain a player id are dropped
    let mut conn = Connection::open(DATABASE_FILE_LOC)?;
    let players : Vec<player_record::PlayerRecord> = draft_selections.into_iter().filter_map(|d| player_record::PlayerRecord::from_partial_draftselection(d)).collect();

    // Updates existing 'player' table with draft information by playerid
    // TODO
// 
    let transaction = conn.transaction()?;
    for p in players.iter() {
        transaction.execute("UPDATE player SET draft_age = ?, draft_year = ?, round = ?, overall = ? WHERE id = ?", 
        params![ p.draft_age, p.draft_year, p.draft_round, p.draft_overall, p.id])?;
    }

    transaction.commit()?;
    Ok(())
}

// Inserts Vec of PlayerSeason information into local database 
// Converts: EP 'PlayerSeason' to Pareto 'PlayerSeasonRecord'
pub fn insert_player_seasons(player_seasons: Vec<player_season::PlayerSeason>) -> rusqlite::Result<()>{

    // Converts 'PlayerSeason's to 'PlayerSeasonRecord's
    let player_seasons : Vec<player_season_record::PlayerSeasonRecord> = player_seasons.into_iter().map(|pss| player_season_record::PlayerSeasonRecord::from(pss)).collect();
    
    // Creates parameter list from PlayerSeasonRecord for SQL insertion
    let mut params = Vec::new();
    for pss in player_seasons.iter() {
        params.push(batch_params!(pss.id, pss.player_id, pss.team_id, pss.season_start_year, pss.gp, pss.g, pss.a, pss.pts, pss.ppg))
    }
    
    // Insert parameters in batches
    batch_insert_query("INSERT INTO player_season(id, player_id, team_id, season_start_year, games_played, goals, assists, points, points_per_game) VALUES ", params)?;
    Ok(())
}

// Inserts Vec of Team information into local database 
// Converts: EP 'Team' to Pareto 'TeamRecord'
pub fn insert_teams(teams: Vec<team::Team>) -> rusqlite::Result<()>{

    // Converts 'Team's to 'TeamRecord's
    let teams : Vec<team_record::TeamRecord> = teams.into_iter().map(|t| team_record::TeamRecord::from(t)).collect();

    // Creates parameter list from TeamRecord for SQL insertion
    let mut params = Vec::new();
    for t in teams.iter() {
        params.push(batch_params!(t.id, t.name))
    }
    
    // Insert parameters in batches
    batch_insert_query("INSERT INTO team(id, name) VALUES ", params)?;
    Ok(())
}

// Inserts Vec of TeamSeason information into local database 
// Converts: EP 'TeamSeason' to Pareto 'TeamSeasonRecord'
pub fn insert_team_seasons(team_seasons: Vec<team_season::TeamSeason>) -> rusqlite::Result<()>{

    // Converts 'TeamSeason's to 'TeamSeasonRecord's
    let team_seasons : Vec<team_season_record::TeamSeasonRecord> = team_seasons.into_iter().map(|t| team_season_record ::TeamSeasonRecord::from(t)).collect();

    // Creates parameter list from TeamSeasonRecord for SQL insertion
    let mut params = Vec::new();
    for tr in team_seasons.iter() {
        params.push(batch_params!(tr.id, tr.team_id, tr.league_slug, tr.season_start_year, tr.group, tr.gp, tr.gf, tr.ga, tr.gd, tr.w, tr.l, tr.t, tr.pts, tr.ppg))
    }

    // Insert parameters in batches
    batch_insert_query("INSERT INTO team_season(id, team_id, league_slug, season_start_year, group_name, games_played, goals_for, goals_against, goal_difference, wins, losses, ties, points, points_per_game) VALUES ", params)?;
    Ok(())
}

// Inserts Vec of League information into local database 
// Converts: EP 'League' to Pareto 'LeagueRecord'
pub fn insert_leagues(leagues: Vec<league::League>) -> rusqlite::Result<()>{

    // Converts 'League's to 'LeagueRecord's
    let leagues : Vec<league_record::LeagueRecord> = leagues.into_iter().map(|l| league_record::LeagueRecord::from(l)).collect();

    // Creates parameter list from LeagueRecord for SQL insertion
   let mut params = Vec::new();
    for l in leagues.iter() {
        params.push(batch_params!(l.slug, l.name, l.league_tier))
    }

    // Insert parameters in batches
    batch_insert_query("INSERT INTO league(slug, name, league_tier) VALUES ", params)?;
    Ok(())
}


// Inserts a large number of records in batches, for a generic table/insertion query
// base_query => insertion query to perform in batches.
//    eg: "INSERT INTO table(v1, v2, v3) VALUES "
// values => Vector of records information to insert. Each element is a Vector of parameters to insert for a record.
//           Each element can be generated by the macro 'batch_params!(x,y,z)'
//    eg: vec![
//          batch_params!(record_1_v1,record_1_v2),
//          batch_params!(record_2_v1,record_2_v2),
//        ]
pub fn batch_insert_query(base_query : &str, values : Vec<Vec<&dyn ToSql>>) -> rusqlite::Result<()> {
    // Ensure batch has at least 1 record
    let num_records = values.len();
    if num_records <= 0 {
        return Ok(()) // an insert query with no record changes nothing
    }

    // Ensures batch has at least 1 parameter
    let num_params = values[0].len();
    if num_params <= 0 {
        return Ok(()) 
    }

    // Iterate over batches of BATCH_MAX_SIZE
    let num_batches = (num_records as f32 / BATCH_MAX_SIZE as f32).ceil() as usize;
    for b in 0..num_batches {
        // Get batch 'b'
        let from_ind = b*BATCH_MAX_SIZE;
        let to_ind = cmp::min((b+1)*BATCH_MAX_SIZE,num_records);

        // Create batch of parameters
        let batch = values[from_ind..to_ind].to_owned() ;
        let batch_size = batch.len();
        let query_params : Vec<&dyn ToSql> = batch.into_iter().flatten().collect();

        // generates (?,?,?,?),(?,?,?,?)... with 'num_params' many '?', a total of 'num_records' times
        let s1 = "?";
        let s2 = ",?".repeat(num_params-1);
        let param_str = format!("({s1}{s2})"); 
        let v1 = &param_str;
        let v2 = format!(",{param_str}").repeat(batch_size-1);
        let values_str = format!("{v1}{v2}"); 
        let query_str = format!("{base_query} {values_str}");

        // Loads cached query from built string and executes
        let conn = Connection::open(DATABASE_FILE_LOC)?;
        let mut cached_query = conn.prepare_cached(&query_str)?;
        cached_query.execute(&*query_params)?;

    }
    Ok(())
}