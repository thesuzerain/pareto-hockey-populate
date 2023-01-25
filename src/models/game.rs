use serde::{Serialize, Deserialize};
use super::{season::Season, team::Team};

// Game
// EP-API schema representing: 
// - a game between two teams during a season
#[derive(Serialize, Deserialize,PartialEq, PartialOrd, Clone, Debug)]
#[serde(rename_all (deserialize = "camelCase"))]
pub struct Game {
    pub id : u32,
    pub season : Season,
    pub team : Option<Team>,
    pub opponent : Option<Team>,
}
