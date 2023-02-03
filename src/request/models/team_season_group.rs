use serde::{Serialize, Deserialize};

use super::{league::League, team::Team, team_season_stats::TeamSeasonStats, season::Season};

// TeamSeason
// EP-API schema representing: 
// - a given team performing in a given season
#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Clone)]
#[serde(rename_all (deserialize = "camelCase"))]
pub struct TeamSeasonGroup {
    pub id : u32,
    pub season : Season,
    pub team : Team,
    pub league : Option<League>,
    pub group : Option<String>, // a TeamSeason may also identify a specific group, if a team played in different groups for the same season

    pub stats : TeamSeasonStats

}