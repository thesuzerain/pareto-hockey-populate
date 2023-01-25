use serde::{Deserialize, Serialize};

// TeamSeasonStats
// EP-API schema representing: 
// - aggregate statistics for a certain team's performance, during a certain season
#[derive(Serialize, Deserialize,PartialEq, PartialOrd, Clone, Debug)]
#[serde(rename_all (deserialize = "camelCase"))]
pub struct TeamSeasonStats {
    pub gp : Option<u32>,   // games played
    pub g : Option<u32>,    // goals
    pub a : Option<u32>,    // assists
    pub pts : Option<u32>,  // points
    pub ppg : Option<f32>   // points per game
}