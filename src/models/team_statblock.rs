use serde::Serialize;

#[derive(Serialize, PartialEq, PartialOrd, Clone)]
pub struct Team_Statblock {
    pub player_id : u32,
    pub team_id : u32,
    pub group_name : String,
    pub league_name : String,

    pub games_played : u32,
    pub goals_for : u32
}