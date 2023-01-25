use serde::{Serialize, Deserialize};
use super::{player::Player};

// DraftSelection
// EP-API schema representing: 
// - a given player's performing for a certain team, during a certain season
#[derive(Serialize, Deserialize,PartialEq, PartialOrd, Clone, Debug)]
#[serde(rename_all (deserialize = "camelCase"))]
pub struct DraftSelection {
    pub id : u32,
    pub player : Option<Player>,

    pub year : Option<u32>,
    pub round : Option<u32>,
    pub overall : Option<u32>
}
