// pareto-hockey-populate
// A library to create and populate a localized SQL database of records retrieved from EliteProspects API (EP-API) endpoints.
// Repeated calls to online API through available routes is extremely slow and tedious, especially for complicated requests.
// A subset of data is to be cached locally as a database and accessible through SQL for faster access.
// This also allows us to avoid querying redundant information, and easily/quickly perform aggregation and analysis.

// Current functionality of main.rs is to create the database from scratch and populate it from available EP-API endpoints.
#[tokio::main]
async fn main() {

    // pareto_hockey_populate::database::erase::erase_team_season_group().unwrap();
    // pareto_hockey_populate::database::erase::erase_league().unwrap();
    pareto_hockey_populate::database::erase::erase_game_logs().unwrap();

    println!("Loading pareto-hockey-populate...");
    pareto_hockey_populate::database::connect::establish_schema().unwrap();
    println!("Connected."); 

    // TODO: do an early check of auth key.

    // TODO: Create CLI for basic population functions.
    // (TODO: remove unwraps from demo fetches)


     // Fetch player information from EP-API
     // TODO: abstract all Player into one function
    // pareto_hockey_populate::populate::populate_players_partial_players().await.unwrap();
    // pareto_hockey_populate::populate::populate_players_partial_draftselections().await.unwrap();
    // pareto_hockey_populate::database::update::update_calculate_draft_age().unwrap();

    // Fetch league information from EP-API
    // pareto_hockey_populate::populate::populate_leagues().await.unwrap();

    // Fetch player_season information from EP-API
    // pareto_hockey_populate::populate::populate_player_season_partial_stats().await.unwrap();

    // Fetch team information from EP-API
    // pareto_hockey_populate::populate::populate_teams().await.unwrap();

    // Fetch team season information from EP-API
    // pareto_hockey_populate::populate::populate_team_season_group().await.unwrap();

    // Fetch game log information
    pareto_hockey_populate::populate::populate_game_logs_for_existing_player().await.unwrap();


    pareto_hockey_populate::database::update::update_calculate_draft_age().unwrap();
    pareto_hockey_populate::database::update::update_aggregate_team_season_records().unwrap();

    // TODO: other updates

}
