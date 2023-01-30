use futures::future::join_all;
use serde::{de::DeserializeOwned};
use crate::{request::fetch::{self}, database};

// populate_leagues
// Iteratively fetches all 'league' information from EP-API, converts it to LeagueRecords,
// then stores it in local database.
pub async fn populate_leagues() -> rusqlite::Result<()> {
    let fetch_func = &fetch::fetch_leagues;
    let insert_func = &database::insert::insert_leagues;

    populate_generic(fetch_func, insert_func, 3).await?;
    Ok(())
}

// populate_teams
// Iteratively fetches all 'team' information from EP-API, converts it to TeamRecord,
// then stores it in local database.
pub async fn populate_teams() -> rusqlite::Result<()> {
    let fetch_func = &fetch::fetch_teams;
    let insert_func = &database::insert::insert_teams;

    populate_generic(fetch_func, insert_func, 10).await?;
    Ok(())
}


// populate_team_season
// Iteratively fetches all 'league' information from EP-API, converts it to LeagueRecords,
// then stores it in local database.
pub async fn populate_team_seasons() -> rusqlite::Result<()> {
    let fetch_func = &fetch::fetch_team_season;
    let insert_func = &database::insert::insert_team_seasons;

    populate_generic(fetch_func, insert_func, 50).await?;
    Ok(())
}

// populate_players_partial_players
// Iteratively fetches all 'Player' information from EP-API, converts it to PlayerRecords,
// then stores it in local database.
// This function only obtains the information obtainable from EP-API Player model, and does not include DraftSelection information.
pub async fn populate_players_partial_players() -> rusqlite::Result<()> {
    let fetch_func = &fetch::fetch_players;
    let insert_func = &database::insert::insert_player_partial_player;

    populate_generic(fetch_func, insert_func, 50).await?;
    Ok(())
}

// populate_players_partial_draftselections
// Iteratively fetches all 'DraftSelection' information from EP-API, converts it to PlayerRecords,
// then appends it to existing PlayerRecord records in local database.
// This function only obtains the information obtainable from EP-API Player model, and does not obtain basic Player information.
pub async fn populate_players_partial_draftselections() -> rusqlite::Result<()> {
    let fetch_func = &fetch::fetch_draft_selections;
    let insert_func = &database::insert::insert_player_partial_draftselection;

    populate_generic(fetch_func, insert_func,50).await?;
    Ok(())
}

// populate_player_season_partial_stats
// Iteratively fetches all 'Player_Season' information from EP-API, converts it to LeagueRecords,
// then stores it in local database.
// This function only obtains the 'regularStats' obtainable from EP-API PlayerStats model, and does not obtain time-on-ice.
pub async fn populate_player_season_partial_stats() -> rusqlite::Result<()> {
    let fetch_func = &fetch::fetch_player_season;
    let insert_func = &database::insert::insert_player_seasons;

    populate_generic(fetch_func, insert_func, 50).await?;

    Ok(())
}

// TODO: Time on ice
// populate_player_season_partial_toi
// Iteratively fetches all 'GameLogs' information for each EXISTING 'Player', gets the aggregate total TOI,
// then stores it in local database.
// This function only obtains the 'time on ice' obtainable from aggregate EP-API GameLogs model, and appends it to already-existing player_season
// pub async fn populate_player_season_partial_toi() -> rusqlite::Result<()> {

//     // TODO

//     let player_id_list = database::select::select_player_ids()?;

//     let mut fut_list = Vec::new();
//     for player_id in player_id_list {
//         let fut = async move {
//             let game_logs = fetch_game_logs_for_player(player_id, 0, 1, 1).await?;
//             game_logs.into_iter().fold(0, |acc, gl| acc + gl.stats);
//             Ok(())
//         };
//         fut_list.push(fut);
//     }



//     // let fetch_func = &fetch::fetch_player_season;
//     // let insert_func = &database::insert::insert_player_seasons;

//     // populate_generic(fetch_func, insert_func, 50).await?;
//     Ok(())
// }


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
    -> rusqlite::Result<()> 
    where B : futures::Future<Output = reqwest::Result<Vec<T>>> {

    let mut batch_offset = 0;

    // Loop, increasing the batch_offset until no more data is available
    loop {
        // Split range of data into 'total_splits' calls to fetch_callback running simultaenously
        // Each is identifiable by index 'i' (passed to those functions as 'split_id')
        let mut get_futures: Vec<_> = Vec::new();
        for i in 0..total_splits {

            let populate_batch = async move {
                let res = fetch_callback(batch_offset, i, total_splits).await.unwrap();
                let num_records = res.len();
                insert_callback(res)?;
                Ok(num_records)
            }; 
            get_futures.push(populate_batch);
        }
        
        // Rejoin futures together for this batch
        let batch_results : Vec<rusqlite::Result<usize>> = join_all(get_futures).await;
        let num_records_per_batch : rusqlite::Result<Vec<usize>>= batch_results.into_iter().collect();

        // Break if any EP-API calls returned 0 results (meaning we reach the end)
        if num_records_per_batch?.into_iter().filter(|i| *i <= 0).collect::<Vec<usize>>().len() > 0 {
            break;
        } else {
            batch_offset += 1;
        }
    }
    Ok(())
}

