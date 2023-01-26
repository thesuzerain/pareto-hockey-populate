use futures::future::join_all;
use serde::{de::DeserializeOwned};
use crate::{request::fetch, database};

// populate_leagues
// Iteratively fetches all 'league' information from EP-API, converts it to LeagueRecords,
// then stores it in local database.
pub async fn populate_leagues() -> rusqlite::Result<()> {
    let fetch_func = &fetch::fetch_leagues;
    let insert_func = &database::insert::insert_leagues;

    populate_generic(fetch_func, insert_func, 3).await?;
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

    populate_generic(fetch_func, insert_func, 50).await?;
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
    -> rusqlite::Result<()> 
    where B : futures::Future<Output = reqwest::Result<Vec<T>>> {

    let mut finished = false;
    let mut batch_offset = 0;

    // Loop, increasing the batch_offset until no more data is available
    while !finished {

        // Split range of data into 'total_splits' calls to fetch_callback running simultaenously
        // Each is identifiable by index 'i' (passed to those functions as 'split_id')
        let mut get_futures = Vec::new();
        for i in 0..total_splits {
            let res = fetch_callback(batch_offset, i, total_splits);
            get_futures.push(res);
        }
        batch_offset += 1;
        
        // Wait for functions to finish before insertion
        // TODO: restructure such that the data can immediately be inserted, and move join_all to end of function 
        let res_vec = join_all(get_futures).await;
        
        // Insert loaded data from each Future
        for res in res_vec {
            let res : Vec<T> = res.unwrap(); // TODO: abstract error, dont use unwrap
            if res.len() <= 0 {
                finished = true;
            }
            insert_callback(res)?;
        }
    }
    Ok(())

}

