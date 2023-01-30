// use pareto_hockey_populate::fetch_draft_selections;
// use pareto_hockey_populate::fetch_game_logs;
// use pareto_hockey_populate::fetch_leagues;
// use pareto_hockey_populate::fetch_players;
// use pareto_hockey_populate::fetch_player_season;
// use pareto_hockey_populate::fetch_team_season;

// pareto-hockey-populate
// A library to create and populate a localized SQL database of records retrieved from EliteProspects API (EP-API) endpoints.
// Repeated calls to online API through available routes is extremely slow and tedious, especially for complicated requests.
// A subset of data is to be cached locally as a database and accessible through SQL for faster access.
// This also allows us to avoid querying redundant information, and easily/quickly perform aggregation and analysis.

// Current functionality of main.rs is to create the database from scratch and populate it from available EP-API endpoints.
#[tokio::main]
async fn main() {
    println!("Loading pareto-hockey-populate...");

    pareto_hockey_populate::connect_database().unwrap();

    println!("Connected."); 

    // TODO: do an early check of auth key.

    // TODO: Create CLI for basic population functions.
    // (TODO: remove unwraps from demo fetches)

     // Fetch player information from EP-API
     // TODO: abstract all Player into one function
    pareto_hockey_populate::populate::populate_players_partial_players().await.unwrap();
    pareto_hockey_populate::populate::populate_players_partial_draftselections().await.unwrap();
    pareto_hockey_populate::database::update::update_calculate_draft_age().unwrap();

    // Fetch league information from EP-API
    pareto_hockey_populate::populate::populate_leagues().await.unwrap();

    // Fetch player_season information from EP-API
    pareto_hockey_populate::populate::populate_player_season_partial_stats().await.unwrap();

    // Fetch team information from EP-API
    pareto_hockey_populate::populate::populate_teams().await.unwrap();

    // Fetch team season information from EP-API
    pareto_hockey_populate::populate::populate_team_seasons().await.unwrap();



    // TODO: other gamelogs

}
