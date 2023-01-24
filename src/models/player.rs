use serde::Serialize;
use chrono::{self, Utc};

#[derive(Serialize, PartialEq, PartialOrd, Clone)]
pub struct Player {
    pub id : u32,
    pub draft_age : u32,
    pub draft_year : u32,
    pub player_name : String,
    
    pub nhl_career_ppg : f32,


    #[serde(with = "ts_seconds_option")]
    pub date_of_birth : chrono::DateTime<Utc>,

}