use database::{connect, models::league_record::LeagueRecord};
use models::{player_season::PlayerSeason, team_season::TeamSeason, player::Player, draft_selection::{DraftSelection, self}, game_log::GameLog, league::League, team::Team};
use request::rest;

mod models;
mod database;
mod request;

// pareto-hockey-populate
// A library to create and populate a localized SQL database of records retrieved from EliteProspects API (EP-API) endpoints.
// Repeated calls to online API through available routes is extremely slow and tedious, especially for complicated requests.
// A subset of data is to be cached locally as a database and accessible through SQL for faster access.
// This also allows us to avoid querying redundant information, and easily/quickly perform aggregation and analysis.

// Auth file containing apiKey (stored locally, do not push to Git)
pub const AUTH_FILE_LOCATION : &'static str = "auth.txt";

pub fn connect_database() -> rusqlite::Result<()> {
    database::connect::connect()
}

pub fn populate_leagues(leagues : Vec<League>) -> rusqlite::Result<()> {
    database::populate::populate_leagues(leagues)
}

pub fn populate_players(players : Vec<Player>, draft_selections : Vec<DraftSelection>) -> rusqlite::Result<()> {
    database::populate::populate_players(players, draft_selections)
}

pub fn populate_player_seasons(player_season : Vec<PlayerSeason>) -> rusqlite::Result<()> {
    database::populate::populate_player_seasons(player_season)
}

pub fn populate_teams(teams : Vec<Team>) -> rusqlite::Result<()> {
    database::populate::populate_teams(teams)
}

pub fn populate_team_seasons(teams_seasons : Vec<TeamSeason>) -> rusqlite::Result<()> {
    database::populate::populate_team_seasons(teams_seasons)
}


// Fetches Vec of all 'Player' objects from EP-API
pub async fn fetch_players() -> Result<Vec<Player>, reqwest::Error> {
    Ok(rest::get_all::<Player>("players").await?)
}

// Fetches Vec of all 'PlayerSeason' objects from EP-API
pub async fn fetch_player_season() -> Result<Vec<PlayerSeason>, reqwest::Error> {
    Ok(rest::get_all::<PlayerSeason>("player-stats").await?)
}

// Fetches Vec of all 'TeamSeason' objects from EP-API
pub async fn fetch_team_season() -> Result<Vec<TeamSeason>, reqwest::Error> {
    Ok(rest::get_all::<TeamSeason>("team-stats").await?)
}

// Fetches Vec of all 'DraftSelection' objects from EP-API
pub async fn fetch_draft_selections() -> Result<Vec<DraftSelection>, reqwest::Error> {
    Ok(rest::get_all::<DraftSelection>("draft-selections").await?)
}

// Fetches Vec of all 'GameLog' objects from EP-API
pub async fn fetch_game_logs() -> Result<Vec<GameLog>, reqwest::Error> {
    Ok(rest::get_all::<GameLog>("game-logs").await?)
}

// Fetches Vec of all 'GameLog' objects from EP-API
pub async fn fetch_leagues() -> Result<Vec<League>, reqwest::Error> {
    Ok(rest::get_all::<League>("leagues").await?)
}

