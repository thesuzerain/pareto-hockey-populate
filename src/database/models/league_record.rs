use crate::database::models::league_tier;

// League
// Pareto schema representing: 
// - a hockey league
pub struct LeagueRecord {
    pub slug : String,
    pub name : String,
    pub league_tier : u32,
    pub logo_url : Option<String>
}

impl LeagueRecord {
    // Converts a EP-API structured League to a local LeagueRecord
    pub fn from(l : crate::request::models::league::League) -> LeagueRecord {
        let league_tier = league_tier::get_league_tier(&l.slug) as u32; // league 'tier' from predefined list
        LeagueRecord {
            slug: l.slug,
            name: l.name,
            league_tier,
            logo_url: l.image_url
        }
    }
}
