use thiserror::Error;

use request::rest;

mod request;
pub mod database;
pub mod populate;

// pareto-hockey-populate
// A library to create and populate a localized SQL database of records retrieved from EliteProspects API (EP-API) endpoints.
// Repeated calls to online API through available routes is extremely slow and tedious, especially for complicated requests.
// A subset of data is to be cached locally as a database and accessible through SQL for faster access.
// This also allows us to avoid querying redundant information, and easily/quickly perform aggregation and analysis.

// Auth file containing apiKey (stored locally, do not push to Git)
pub const AUTH_FILE_LOCATION : &'static str = "auth.txt";

#[derive(Error, Debug)]
pub enum PopulateError {
    #[error("Error while accessing HTTP endpoint: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Error while interacting with the database: {0}")]
    Database(#[from] rusqlite::Error)
}