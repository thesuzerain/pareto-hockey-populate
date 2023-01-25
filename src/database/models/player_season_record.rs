// PlayerSeasonRecord
// Pareto schema representing: 
// - a given player's performance during a given season, playing for a given team
pub struct PlayerSeasonRecord{

    pub id : u32,
    pub player_id : u32,
    pub team_id : u32,
    pub season_start_year : u32,

    pub gp : Option<u32>,   // games played
    pub g : Option<u32>,    // goals
    pub a : Option<u32>,    // assists
    pub pts : Option<u32>,  // points
    pub ppg : Option<f32>   // average points per game
}

impl PlayerSeasonRecord {

    pub fn from(pss : crate::models::player_season::PlayerSeason) -> PlayerSeasonRecord {
        let stats = pss.regular_stats;
        PlayerSeasonRecord {
            id : pss.id,
            player_id: pss.player.id,
            team_id: pss.team.id,
            season_start_year: pss.season.start_year,
            gp: if let Some(ref s) = stats { s.gp } else { None },
            g: if let Some(ref s) = stats { s.g } else { None },
            a: if let Some(ref s) = stats { s.a } else { None },
            pts: if let Some(ref s) = stats { s.pts } else { None },
            ppg: if let Some(ref s) = stats { s.ppg } else { None }
        }
    }

}