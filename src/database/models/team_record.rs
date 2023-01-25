// TeamRecord
// Pareto schema representing: 
// - a given team

pub struct TeamRecord {
    pub id : u32,
    pub name : String
}

impl TeamRecord {

    pub fn from(t : crate::models::team::Team) -> TeamRecord {
        TeamRecord {
            id: t.id,
            name: t.name
        }
    }
}