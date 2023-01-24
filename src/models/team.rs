
use serde::Serialize;

#[derive(Serialize,PartialEq,PartialOrd,Clone)]
pub struct Team {
    pub id : u32,
    pub team_name : String
}