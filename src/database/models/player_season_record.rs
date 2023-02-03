// PlayerSeasonRecord
// Pareto schema representing: 
// - a given player's performance during a given season, playing for a given team
pub struct PlayerSeasonRecord{

    pub id : u32,
    pub player_id : Option<u32>,
    pub team_id : Option<u32>,
    pub season_start_year : Option<u32>,
    pub league_slug : Option<String>,

    pub gp : Option<u32>,   // games played
    pub g : Option<u32>,    // goals
    pub a : Option<u32>,    // assists
    pub pts : Option<u32>,  // points
}

impl PlayerSeasonRecord {

    pub fn from(pss : crate::request::models::player_season::PlayerSeason) -> PlayerSeasonRecord {
        let stats = pss.regular_stats;
        PlayerSeasonRecord {
            id : pss.id,
            player_id: if let Some(p) = pss.player { Some(p.id) } else { None },
            team_id: if let Some(t) = pss.team { Some(t.id) } else { None },
            league_slug: if let Some(l) = pss.league { Some(l.slug) } else { None },
            season_start_year: if let Some(season) = pss.season { Some(season.start_year) } else { None },
            gp: if let Some(ref s) = stats { s.gp } else { None },
            g: if let Some(ref s) = stats { s.g } else { None },
            a: if let Some(ref s) = stats { s.a } else { None },
            pts: if let Some(ref s) = stats { s.pts } else { None },
        }
    }
}