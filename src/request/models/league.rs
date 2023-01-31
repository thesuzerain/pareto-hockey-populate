use serde::{Serialize, Deserialize};

// League
// EP-API schema representing: 
// - a team's League that they play in for a given season
#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Clone)]
#[serde(rename_all (deserialize = "camelCase"))]
pub struct League {
    pub slug : String,
    pub name : String
}