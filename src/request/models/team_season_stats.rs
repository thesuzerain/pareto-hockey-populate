use serde::{Deserialize, Serialize};

// TeamSeasonStats
// EP-API schema representing: 
// - aggregate statistics for a certain team's performance, during a certain season
#[derive(Serialize, Deserialize,PartialEq, PartialOrd, Clone, Debug)]
#[serde(rename_all (deserialize = "UPPERCASE"))]
pub struct TeamSeasonStats {
    pub gp : Option<u32>,   // games played
    pub w : Option<u32>,    // wins
    pub l : Option<u32>,    // losses
    pub t : Option<u32>,    // ties
    pub gf : Option<u32>,    // goals for
    pub ga : Option<u32>,    // goals against
    pub pts : Option<u32>,  // points
    pub gd : Option<i32>,  // goal difference
}