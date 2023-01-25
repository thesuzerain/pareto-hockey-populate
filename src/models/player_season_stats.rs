use serde::{Deserialize, Serialize};

// PlayerSeasonStats
// EP-API schema representing: 
// - the stats of a given player's performance during a given season, playing for a given team
#[derive(Serialize, Deserialize,PartialEq, PartialOrd, Clone, Debug)]
#[serde(rename_all (deserialize = "camelCase"))]
pub struct PlayerSeasonStats{
    pub gp : Option<u32>,   // games played
    pub g : Option<u32>,    // goals
    pub a : Option<u32>,    // assists
    pub pts : Option<u32>,  // points
    pub ppg : Option<f32>   // average points per game
}