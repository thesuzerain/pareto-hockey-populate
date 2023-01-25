// League
// Pareto schema representing: 
// - a hockey league
pub struct LeagueRecord {
    pub slug : String,
    pub name : String,
    pub league_tier : u32
}

impl LeagueRecord {
    // Converts a EP-API structured League to a local LeagueRecord
    pub fn from(l : crate::models::league::League) -> LeagueRecord {
        LeagueRecord {
            slug: l.slug,
            name: l.name,
            league_tier: 0 // TODO: league tiers
        }
    }
}
