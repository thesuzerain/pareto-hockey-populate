use serde::{Serialize, Deserialize};
use super::{player::Player, team::Team, season::Season, player_season_stats::PlayerSeasonStats};

// PlayerSeason
// EP-API schema representing: 
// - a given player's performing for a certain team, during a certain season
#[derive(Serialize, Deserialize,PartialEq, PartialOrd, Clone, Debug)]
#[serde(rename_all (deserialize = "camelCase"))]
pub struct PlayerSeason {
    pub id : u32,
    pub player : Option<Player>,
    pub team : Option<Team>,
    pub season : Option<Season>,
    pub regular_stats : Option<PlayerSeasonStats>
}
