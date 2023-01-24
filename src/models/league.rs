use serde::Serialize;


#[derive(Serialize, PartialEq, PartialOrd)]
pub struct League {
    pub slug : String,
    pub tier : u32
}