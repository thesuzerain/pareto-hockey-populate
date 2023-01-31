use serde::{Serialize, Deserialize};
use super::{player::Player, game_log_stats::GameLogStats, game::Game};

// GameLog
// EP-API schema representing: 
// - a given player's performing for a certain team, during a certain season
#[derive(Serialize, Deserialize,PartialEq, PartialOrd, Clone, Debug)]
#[serde(rename_all (deserialize = "camelCase"))]
pub struct GameLog {
    pub id : u32,
    pub game : Game,
    pub player : Option<Player>,
    pub stats : Option<GameLogStats>,
    pub team_score : u32,
    pub opponent_score : u32,

}
