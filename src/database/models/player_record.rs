use chrono::{NaiveDate, Datelike};

use crate::models::{position::Position, draft_selection};

// PlayerRecord
// Pareto schema representing: 
// - a given player
pub struct PlayerRecord {
    pub id : u32,

    pub name : String,
    pub position : Position,

    pub draft_age : Option<u32>,
    pub draft_year : Option<u32>,
    pub round : Option<u32>,
    pub overall : Option<u32>,

    pub date_of_birth : String
}

impl PlayerRecord {
    // Converts a EP-API structured Player and a DraftSelection to a local Pareto PlayerRecord
    pub fn from(p : crate::models::player::Player, d : Option<&crate::models::draft_selection::DraftSelection>) -> PlayerRecord {

        // If full name doesnt exist, try to recreate it from first+last
        let name = if let Some(n) = p.name {n} else {
            format!("{} {}", p.first_name.unwrap_or_default(), p.last_name.unwrap_or_default() )
        };

        let date_of_birth = p.date_of_birth.unwrap_or_default();
        let birth_year = match NaiveDate::parse_from_str(&date_of_birth, "%Y-%m-%d") {
            Ok(date) => date.year() as u32,
            Err(_) => 0
        };

        let position = p.position.unwrap_or(Position::None);
        
        if let Some(d) = d {
            
            let draft_year = d.year.unwrap_or_default();
            let draft_age = if draft_year > 0 && birth_year > 0 { Some(draft_year - birth_year) } else { None };

            PlayerRecord {
                id: p.id,
                name,
                position,
                date_of_birth,

                draft_age,
                draft_year: d.year,
                round : d.round,
                overall : d.overall,
            }
        } else {
            PlayerRecord {
                id: p.id,
                name,
                position,
                date_of_birth,

                draft_age: None,
                draft_year: None,
                round : None,
                overall : None,
            }
        }    
    }


}
