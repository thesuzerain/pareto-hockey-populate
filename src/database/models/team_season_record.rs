use crate::models::team_season::TeamSeason;


// TeamSeason
// Pareto schema representing: 
// - a given team's performance and aggregate stats in a given season
pub struct TeamSeasonRecord {
    pub id : u32,

    pub team_id : u32,
    pub league_slug : String,
    pub season_start_year : u32,

    pub group : Option<String>, // a TeamSeason may also identify a specific group, if a team played in different groups for the same season

    pub gp : Option<u32>,   // games played
    pub g : Option<u32>,    // goals
    pub a : Option<u32>,    // assists
    pub pts : Option<u32>,  // points
    pub ppg : Option<f32>   // points per game
}

impl TeamSeasonRecord {

    pub fn from(ts : TeamSeason) -> TeamSeasonRecord {
        TeamSeasonRecord { 
            id: ts.id,
            team_id: ts.team.id, 
            league_slug: ts.league.slug, 
            season_start_year: ts.season.start_year, 
            group: ts.group, 
            gp: ts.stats.gp, 
            g: ts.stats.g, 
            a: ts.stats.a, 
            pts: ts.stats.pts, 
            ppg: ts.stats.ppg 
        }
    }

}