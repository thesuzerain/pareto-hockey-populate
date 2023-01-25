use serde::{Serialize, Deserialize};
use super::position::Position;

// Player
// EP-API schema representing: 
// - a given player
#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Clone, Debug)]
#[serde(rename_all (deserialize = "camelCase"))]
pub struct Player {

    pub id : u32,

    pub name : Option<String>,
    pub position : Option<Position>,
    pub first_name : Option<String>,
    pub last_name : Option<String>,

    pub draft_age : Option<u32>,
    pub draft_year : Option<u32>,

    pub date_of_birth : Option<String> // some dates on API are improperly formatted

}