
use serde::{Serialize, Deserialize};

// Team
// EP-API schema representing: 
// - a given team
#[derive(Serialize,Deserialize,PartialEq,PartialOrd,Clone, Debug)]
#[serde(rename_all (deserialize = "camelCase"))]
pub struct Team {
    pub id : u32,
    pub name : String,
    pub logo_url : Option<String>
}