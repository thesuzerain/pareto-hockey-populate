// TeamRecord
// Pareto schema representing: 
// - a given team

pub struct TeamRecord {
    pub id : u32,
    pub name : String,
    pub logo_url : Option<String>
}

impl TeamRecord {

    pub fn from(t : crate::request::models::team::Team) -> TeamRecord {
        TeamRecord {
            id: t.id,
            name: t.name,
            logo_url: t.logo_url
        }
    }
}