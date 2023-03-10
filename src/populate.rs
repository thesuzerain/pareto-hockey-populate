use futures::future::join_all;
use reqwest::StatusCode;
use serde::{de::DeserializeOwned};
use crate::{request::fetch, database, PopulateError};


// Number of consecutive times to populating a batch when a 504 GATEWAY TIMEOUT is returned
const MAX_FETCH_ATTEMPTS : usize = 5;


// populate_leagues
// Iteratively fetches all 'league' information from EP-API, converts it to LeagueRecords,
// then stores it in local database.
pub async fn populate_leagues() -> Result<(), PopulateError> {
    let fetch_func = &fetch::fetch_leagues;
    let insert_func = &database::insert::insert_leagues;

    populate_generic(fetch_func, insert_func, 3).await?;
    Ok(())
}

// populate_teams
// Iteratively fetches all 'team' information from EP-API, converts it to TeamRecord,
// then stores it in local database.
pub async fn populate_teams() -> Result<(), PopulateError> {
    let fetch_func = &fetch::fetch_teams;
    let insert_func = &database::insert::insert_teams;

    populate_generic(fetch_func, insert_func, 10).await?;
    Ok(())
}


// populate_team_season
// Iteratively fetches all 'league' information from EP-API, converts it to LeagueRecords,
// then stores it in local database.
pub async fn populate_team_season_group() -> Result<(), PopulateError> {
    let fetch_func = &fetch::fetch_team_season_group;
    let insert_func = &database::insert::insert_team_season_group;

    populate_generic(fetch_func, insert_func, 50).await?;
    Ok(())
}

// populate_players_partial_players
// Iteratively fetches all 'Player' information from EP-API, converts it to PlayerRecords,
// then stores it in local database.
// This function only obtains the information obtainable from EP-API Player model, and does not include DraftSelection information.
pub async fn populate_players_partial_players() -> Result<(), PopulateError> {
    let fetch_func = &fetch::fetch_players;
    let insert_func = &database::insert::insert_player_partial_player;

    populate_generic(fetch_func, insert_func, 50).await?;
    Ok(())
}

// populate_players_partial_draftselections
// Iteratively fetches all 'DraftSelection' information from EP-API, converts it to PlayerRecords,
// then appends it to existing PlayerRecord records in local database.
// This function only obtains the information obtainable from EP-API Player model, and does not obtain basic Player information.
pub async fn populate_players_partial_draftselections() -> Result<(), PopulateError> {
    let fetch_func = &fetch::fetch_draft_selections;
    let insert_func = &database::insert::insert_player_partial_draftselection;

    populate_generic(fetch_func, insert_func,50).await?;
    Ok(())
}

// populate_player_season_partial_stats
// Iteratively fetches all 'Player_Season' information from EP-API, converts it to LeagueRecords,
// then stores it in local database.
// This function only obtains the 'regularStats' obtainable from EP-API PlayerStats model, and does not obtain time-on-ice.
pub async fn populate_player_season_partial_stats() -> Result<(), PopulateError> {
    let fetch_func = &fetch::fetch_player_season;
    let insert_func = &database::insert::insert_player_seasons;

    populate_generic(fetch_func, insert_func, 50).await?;

    Ok(())
}

// populate_game_logs_for_existing_player
// Iteratively fetches all 'GameLogs' information for each EXISTING 'Player' then stores it in local database.
// We do NOT fetch all game logs- there are rather a lot and any unnecessary ones we don't want.
// TODO: this uses essentially the same logic to batch poulation below: can we abstract?
pub async fn populate_game_logs_for_existing_player() -> Result<(), PopulateError> {

    let player_id_list = database::select::select_player_ids()?;
    let mut batch_offset = 0; 

    // How many players to do at a time
    let player_splits = 50;

    // Loop through 'player_split'-sized groups of players, fetching all game-logs within 
    loop {
            // Create 'player_split' many async functions that fetch game_log data, where player id is offset by batch_offset
        let mut get_futures: Vec<_> = Vec::new();
        for i in 0..player_splits {

            let player_id = player_id_list.get(batch_offset + i);
            if let Some(player_id) = player_id {
                // If player_id exists in database (and we havent gone past the list), create a Future for fetching and storing all its game logs
                let populate_player_batch = async move {
                    let fetch_func = & |batch_offset, i, total_splits| 
                        { fetch::fetch_game_logs_for_player(*player_id, batch_offset, i, total_splits)}; 
                    let insert_func = &database::insert::insert_game_logs;

                    // only one split for inner function, as probably less than 1000 games per player   
                    populate_generic(fetch_func, insert_func, 1).await?;
                    Ok::<(),PopulateError>(())
                }; 
                // Wait for this batch of futures to finish
                get_futures.push(populate_player_batch);
            }
        }
        
        let batch_results : Vec<Result<(),PopulateError>> = join_all(get_futures).await;
        let batch_results : Result<Vec<()>,PopulateError> = batch_results.into_iter().collect();
        let _ = batch_results?;

        // Increment batch offset, and exit loop if done
        batch_offset += player_splits;
        if batch_offset > player_id_list.len() {
            break;
        }
    }
    Ok(())
}

// populate_generic
// Generic function used by populate_leagues, etc
// Calls 'fetch_callback' to download data, 'insert_callback' to convert data to local record struct and insert it into local database. 
// This is repeated iteratively until no more data is retrievable.
// fetch_call back takes 'batch_offset, split_id, total_splits' args (examples can be found in request::fetch)
// insert_call back takes the downloaded data as an argument (examples can be found in database::insert)
pub async fn populate_generic<T : DeserializeOwned, B>(
    fetch_callback : &impl Fn(usize, usize, usize) -> B, //  fetch function to fetch data
    insert_callback : &impl Fn(Vec<T>) -> rusqlite::Result<()>, // insert callback to insert local data
    total_splits : usize) 
    -> Result<(), PopulateError>
    where B : futures::Future<Output = reqwest::Result<Vec<T>>> {

    let mut batch_offset = 0;


    // Loop, increasing the batch_offset until no more data is available
    loop { // todo: could be a 'while'
        // Split range of data into 'total_splits' calls to fetch_callback running simultaenously
        // Each is identifiable by index 'i' (passed to those functions as 'split_id')
        let mut get_futures: Vec<_> = Vec::new();
        for i in 0..total_splits {

            let populate_batch = async move {
              
                // Check status of result
                // If successful, use it.
                // If a 504 error, try again up to 'MAX_FETCH_ATTEMPTS' many times.
                // If other error, propogate up.
                let mut res = Vec::new();
                for _ in 0..MAX_FETCH_ATTEMPTS {
                    res = match fetch_callback(batch_offset, i, total_splits).await {
                        Ok(r) => r,
                        Err(e) => {
                            match e.status() {
                                // continue to try loop again if 504
                                Some( StatusCode::GATEWAY_TIMEOUT) => continue,
                                _ => return Err(PopulateError::Http(e))
                            }
                        },
                    };
                    break;                
                }

                let num_records = res.len();

                // Runs 'insert' function to convert and insert fetched records
                if num_records > 0 {
                    insert_callback(res)?;
                }
                Ok(num_records)
            }; 
            get_futures.push(populate_batch);
        }
        
        // Rejoin futures together for this batch
        let batch_results : Vec<Result<usize, PopulateError>> = join_all(get_futures).await;
        let num_records_per_batch : Result<Vec<usize>, PopulateError>= batch_results.into_iter().collect();


        // Break if any EP-API calls returned 0 results (meaning we reach the end)
        if num_records_per_batch?.into_iter().filter(|i| *i <= 0).collect::<Vec<usize>>().len() > 0 {
            break;
        } else {
            batch_offset += 1;
        }
    }
    Ok(())
}

