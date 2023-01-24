use serde::Serialize;

#[derive(Serialize, PartialEq, PartialOrd, Clone)]
pub struct PlayerTeamStatblock {
    pub player_id : u32,
    pub team_statblock_id : u32,
    pub group_name : String
}