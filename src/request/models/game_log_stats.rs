use serde::{Deserialize, Serialize};

// GameLogStats
// EP-API schema representing: 
// - the stats of a given player's performance during a given game
#[derive(Serialize, Deserialize,PartialEq, PartialOrd, Clone, Debug)]
#[serde(rename_all (deserialize = "UPPERCASE"))]
pub struct GameLogStats{
    pub g : Option<i32>,    // goals
    pub a : Option<i32>,    // assists
    pub toi : Option<String> // time on ice
}