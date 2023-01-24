use serde::Serialize;

#[derive(Serialize, Clone, PartialEq, PartialOrd)]
pub struct Cohort {
    pub cid : u32,
    pub age : u32,
    pub season_slug : String,
    pub league_tier : u32
}