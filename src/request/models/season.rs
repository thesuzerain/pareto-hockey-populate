use serde::{Serialize, Deserialize};

// Season
// EP-API schema representing: 
// - a given season of play, with start and end year
#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Clone, Debug)]
#[serde(rename_all (deserialize = "camelCase"))]
pub struct Season {
    pub start_year : u32,
    pub end_year : u32,
}