use crate::request::models::team_season::TeamSeason;


// TeamSeason
// Pareto schema representing: 
// - a given team's performance and aggregate stats in a given season
pub struct TeamSeasonRecord {
    pub id : u32,

    pub team_id : u32,
    pub league_slug : Option<String>,
    pub season_start_year : u32,

    pub group : Option<String>, // a TeamSeason may also identify a specific group, if a team played in different groups for the same season

    pub gp : Option<u32>,   // games played
    pub w : Option<u32>,    // wins
    pub l : Option<u32>,    // losses
    pub t : Option<u32>,    // ties
    pub gf : Option<u32>,    // goals for
    pub ga : Option<u32>,    // goals against
    pub pts : Option<u32>,  // points
    pub gd : Option<i32>,  // goal difference
    pub ppg : Option<f32>   // points per game
}

impl TeamSeasonRecord {

    pub fn from(ts : TeamSeason) -> TeamSeasonRecord {
        TeamSeasonRecord { 
            id: ts.id,
            team_id: ts.team.id, 
            league_slug: if let Some(l) = ts.league {Some(l.slug)} else {None}, 
            season_start_year: ts.season.start_year, 
            group: ts.group, 
            gp: ts.stats.gp, 
            w: ts.stats.w, 
            l: ts.stats.l, 
            t: ts.stats.t, 
            gf: ts.stats.gf, 
            ga: ts.stats.ga, 
            gd: ts.stats.gd, 

            pts: ts.stats.pts, 
            ppg: ts.stats.ppg 
        }
    }

}