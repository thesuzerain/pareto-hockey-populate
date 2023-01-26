use crate::models::{position::Position};

// PlayerRecord
// Pareto schema representing: 
// - a given player
// Unlike other records, this one is composed of two separate models from the EP-API database. 
// A PlayerRecord can be constructed from either group, in case only some data is missing (eg: a player who never gets drafted, which is still relevant). 
pub struct PlayerRecord {
    pub id : u32,

    // From models::player::Player
    pub name : Option<String>,
    pub position : Option<Position>,
    pub date_of_birth : Option<String>,

    // From models::draft_selection::DraftSelection
    pub draft_year : Option<u32>,
    pub draft_round : Option<u32>,
    pub draft_overall : Option<u32>,

    // From both models
    pub draft_age : Option<u32>,

}

impl PlayerRecord {
    // Converts a EP-API structured Player to a partial local Pareto PlayerRecord (without EP-API draft information)
    pub fn from_partial_player(p : crate::models::player::Player) -> PlayerRecord {

        // If full name doesnt exist, try to recreate it from first+last
        let name = Some(if let Some(n) = p.name {n} else {
            format!("{} {}", p.first_name.unwrap_or_default(), p.last_name.unwrap_or_default() )
        });

        let date_of_birth = Some(p.date_of_birth.unwrap_or_default());
        let position = Some(p.position.unwrap_or(Position::None));
    
        PlayerRecord {
            id: p.id,
            name,
            position,
            date_of_birth,

            draft_year: None, // added by DraftSelection
            draft_round : None, // added by DraftSelection
            draft_overall : None, // added by DraftSelection

            draft_age: None, // calculated when date_of_birth and draft_year are present

        }
    }

    // Converts a EP-API structured DraftSelection to a possible partial local Pareto PlayerRecord (without EP-API player information)
    // player-id is a required field to build this, but may not be present in a DraftSelection model from the EP-API (for unknown reasons).
    // If player.id is None, then PlayerRecord will also be None as this is not a player.
    pub fn from_partial_draftselection(d : crate::models::draft_selection::DraftSelection) -> Option<PlayerRecord> {
        
        if let Some(player) = d.player {
            Some(PlayerRecord {
                id: player.id,
    
                name: None, // added by Player
                position: None, // added by Player
                date_of_birth: None, // added by Player
    
                draft_year: d.year,
                draft_round : d.round,
                draft_overall : d.overall,
    
                draft_age: None, // calculated when date_of_birth and draft_year are present
            })
        } else {
            None
        }


    }
    

}
