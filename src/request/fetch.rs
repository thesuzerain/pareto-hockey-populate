use serde::de::DeserializeOwned;

use crate::models::{draft_selection::DraftSelection, player::Player, player_season::PlayerSeason, team_season::TeamSeason, game_log::GameLog, league::League};
use crate::rest;

// Fetches Vec of all 'PlayerSeason' objects from EP-API
// Multiple ('total_splits') instances of this can be run asynchronously, where split_id uniquely identifies this split.
// batch_offset offsets all splits by batch_offset * total_splits * MAX_REQ_LIMIT
pub async fn fetch_player_season(batch_offset : usize, split_id: usize, total_splits: usize) -> Result<Vec<PlayerSeason>, reqwest::Error> {
    Ok(fetch_generic("player-stats", Vec::new(), batch_offset, split_id, total_splits).await?)
}

// Fetches Vec of all 'TeamSeason' objects from EP-API
// Multiple ('total_splits') instances of this can be run asynchronously, where split_id uniquely identifies this split.
// batch_offset offsets all splits by batch_offset * total_splits * MAX_REQ_LIMIT
pub async fn fetch_team_season(batch_offset : usize, split_id: usize, total_splits: usize) -> Result<Vec<TeamSeason>, reqwest::Error> {
    Ok(fetch_generic("team-stats", Vec::new(), batch_offset, split_id, total_splits).await?)
}


// Fetches Vec of all 'GameLog' objects from EP-API
// Multiple ('total_splits') instances of this can be run asynchronously, where split_id uniquely identifies this split.
// batch_offset offsets all splits by batch_offset * total_splits * MAX_REQ_LIMIT
pub async fn fetch_game_logs(batch_offset : usize, split_id: usize, total_splits: usize) -> Result<Vec<GameLog>, reqwest::Error> {
    Ok(fetch_generic("game-logs", Vec::new(), batch_offset, split_id, total_splits).await?)
}

// Fetches Vec of all 'League' objects from EP-API
// Multiple ('total_splits') instances of this can be run asynchronously, where split_id uniquely identifies this split.
// batch_offset offsets all splits by batch_offset * total_splits * MAX_REQ_LIMIT
pub async fn fetch_leagues(batch_offset : usize, split_id: usize, total_splits: usize) -> Result<Vec<League>, reqwest::Error> {
    Ok(fetch_generic("leagues", Vec::new(), batch_offset, split_id, total_splits).await?)
}

// Fetches Vec of all 'Player' objects from EP-API
// Multiple ('total_splits') instances of this can be run asynchronously, where split_id uniquely identifies this split.
// batch_offset offsets all splits by batch_offset * total_splits * MAX_REQ_LIMIT
pub async fn fetch_players(batch_offset : usize, split_id: usize, total_splits: usize) -> Result<Vec<Player>, reqwest::Error> {
    Ok(fetch_generic("players", Vec::new(), batch_offset, split_id, total_splits).await?)
}

// Fetches Vec of all 'DraftSelection' objects from EP-API
// Multiple ('total_splits') instances of this can be run asynchronously, where split_id uniquely identifies this split.
// batch_offset offsets all splits by batch_offset * total_splits * MAX_REQ_LIMIT
pub async fn fetch_draft_selections(batch_offset : usize, split_id: usize, total_splits: usize) -> Result<Vec<DraftSelection>, reqwest::Error> {
    Ok(fetch_generic("draft-selections", Vec::new(), batch_offset, split_id, total_splits).await?)
}

// Generic fetch function to 'GET' all data from an endpoint
// It is intended to be used iteratively, by increasing 'batch_offset' to fetch offset data until no more is returned.
// In addition, to run mutlple requests at once, 'split_id' and 'total_splits' can be used to split the data requested across multiple calls to this function.
// eg: if batch_offset is 4 and total_splits is 10, the split_id=0 call will fetch 40_000-41_000, split_id=1 will fetch 41_000-42_000, etc. 
// endpoint => GET endpoint to access
//             eg. "/draft-selections"
// extra_fields => Vec of fields to append
//             eg. vec!["id=50232","league=NHL"]
// batch_offset => number of requests so far; the offset of fetchable data as a multiple of MAX_REQ_LIMIT*total_splits
// split_id => index of the split being requested for this function call
// total_splits => how many splits of data. Must be more than split_id
pub async fn fetch_generic<T : DeserializeOwned>(endpoint : &str, extra_fields : Vec<&str>, batch_offset : usize, split_id : usize, total_splits : usize) -> Result<Vec<T>, reqwest::Error> {
    if split_id > total_splits {
        panic!("fetch_generic split_id cannot exceed maximum splits"); // described in func. contract
    }
    let limit_field = format!("limit={}", rest::MAX_REQ_LIMIT);
    let updated_at_field = format!("sort=-updatedAt");

    let offset = batch_offset*total_splits*rest::MAX_REQ_LIMIT + rest::MAX_REQ_LIMIT * split_id;
    let offset_field = format!("offset={offset}");

    let mut fields_temp = vec![limit_field, updated_at_field, offset_field];
    fields_temp.extend(extra_fields.iter().map(|s| s.to_string()));

    Ok(rest::get_with_fields::<T>(endpoint,fields_temp).await?)
}
