use serde::{Deserialize, Serialize};

// GameLogStats
// EP-API schema representing: 
// - the stats of a given player's performance during a given game
#[derive(Serialize, Deserialize,PartialEq, PartialOrd, Clone, Debug)]
#[serde(rename_all (deserialize = "camelCase"))]
pub struct GameLogStats{
    pub gp : Option<u32>,   // games played
    pub g : Option<u32>,    // goals
    pub a : Option<u32>,    // assists
    pub pts : Option<u32>,  // points
}