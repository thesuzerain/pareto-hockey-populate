use pareto_hockey_populate::fetch_players;
use pareto_hockey_populate::fetch_player_season;
use pareto_hockey_populate::fetch_team_season;

// pareto-hockey-populate
// A library to create and populate a localized SQL database of records retrieved from EliteProspects API (EP-API) endpoints.
// Repeated calls to online API through available routes is extremely slow and tedious, especially for complicated requests.
// A subset of data is to be cached locally as a database and accessible through SQL for faster access.
// This also allows us to avoid querying redundant information, and easily/quickly perform aggregation and analysis.

// Current functionality of main.rs is to create the database from scratch and populate it from available EP-API endpoints.
#[tokio::main]
async fn main() {
    println!("Loading pareto-hockey-populate...");

    // TODO: create local database.

    // TODO: do an early check of auth key.

    // (TODO: remove unwraps from demo fetches)
    // Fetch player seasons from EP-API
    let player_seasons = fetch_player_season().await.unwrap();
    dbg!(player_seasons.len());

    // Fetch team seasons from EP-API
    let team_seasons = fetch_team_season().await.unwrap();
    dbg!(team_seasons.len());

    // Fetch player information from EP-API
    let players = fetch_players().await.unwrap();
    dbg!(players.len());

    // TODO: store fetched records locally.    
}
