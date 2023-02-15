use crate::request::models::game_log_stats::GameLogStats;
use lazy_static::lazy_static;
use regex::Regex;

// GameLogRecord
// Pareto schema representing: 
// - a given player's performance during a certain game
pub struct GameLogRecord{

    // ID of this particular game log
    pub id : u32,

    // Components of player-season to identify it
    // Values are all required
    pub player_id : u32,
    pub season_start_year : u32,
    pub league_slug : String,
    pub team_id : u32,

    pub oppteam_id : u32, 

    pub g : Option<u32>,    // goals
    pub a : Option<u32>,    // assists
    pub toi_secs : Option<u32>, // time on ice

    pub team_score : u32,
    pub opp_score : u32
}

impl GameLogRecord {

    // Attempts to convert a EP-API structured GameLog to a GameLogRecord
    // If TOI value is not convertible, or crucial data is missing, the record is dropped and is None
    // If player.id is None, then PlayerRecord will also be None as this is not a player.

    pub fn from(gl : crate::request::models::game_log::GameLog) -> Option<GameLogRecord> {

        let toi_secs;
        if let Some(GameLogStats { toi : Some(ref toi), ..} ) = gl.stats {
            toi_secs = GameLogRecord::get_toi_as_secs(toi).ok(); // result currenlty unused
        } else {
            toi_secs = None;
        }

        Some(GameLogRecord { 
            id: gl.id,

            player_id: gl.player.id, 
            season_start_year: gl.game.season.start_year, 
            league_slug: gl.game.league.slug, 
            team_id : gl.team.id, 
            oppteam_id: gl.opponent.id, 
            g: if let Some(GameLogStats { g : Some(g), ..} ) = gl.stats { Some(g) } else { None }, 
            a: if let Some(GameLogStats { a : Some(a), ..} ) = gl.stats { Some(a) } else { None }, 
            toi_secs, 
            team_score: gl.team_score,
            opp_score: gl.opponent_score, 
        })
    }

    // Gets time-on-ice clocck string (H:M:S) as a u32 of seconds
    // Currently returns a Result as it attempts to match to Regex.
    // (Result currently not handled as and game log simply dropped, but may be in the future for debugging)
    fn get_toi_as_secs(toi : &str) -> Result<u32,regex::Error> {
        lazy_static ! {
            // Match a wide array of clock syntaxes, which could be H:M:S in many ways:
            // 1:23:33
            // 1232:11
            // hours is not mandatory ,but at least M:S shiouold exist
            static ref CLOCK_REGEX : Regex = Regex::new(r"^(?:(\d+):)?(\d+):(\d+)").unwrap();
        }
        
        let toi_error = || regex::Error::Syntax(format!("Could not match TOI to clock syntax: {toi}."));
        let caps = CLOCK_REGEX.captures(&toi).ok_or_else(toi_error)?;

        // unwrap parsed values, as if they exist they are guaranteed to be \d
        let hours : u32 = if let Some(h) = caps.get(1) {h.as_str().parse().unwrap()} else { 0 };  
        let minutes : u32 = caps.get(2).ok_or_else(toi_error)?.as_str().parse().unwrap();
        let seconds : u32 = caps.get(3).ok_or_else(toi_error)?.as_str().parse().unwrap();

        Ok(hours * 3600 + minutes * 60 + seconds )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_toi_to_seconds() {
        assert_eq!(GameLogRecord::get_toi_as_secs("0:0:0").unwrap(),0);
        assert_eq!(GameLogRecord::get_toi_as_secs("0:0:1").unwrap(),1);
        assert_eq!(GameLogRecord::get_toi_as_secs("0:1:1").unwrap(),60+1);
        assert_eq!(GameLogRecord::get_toi_as_secs("1:1:1").unwrap(),3600 + 60 + 1);
        assert_eq!(GameLogRecord::get_toi_as_secs("1:1:11").unwrap(),3600 + 60 + 11);
        assert_eq!(GameLogRecord::get_toi_as_secs("1:11:1").unwrap(),3600 + 60*11 + 1);
        assert_eq!(GameLogRecord::get_toi_as_secs("11:1:1").unwrap(),3600*11 + 60 + 1);
        assert_eq!(GameLogRecord::get_toi_as_secs("11:100:1").unwrap(),3600*11 + 60*100 + 1);
        assert_eq!(GameLogRecord::get_toi_as_secs("100:1").unwrap(),60*100 + 1);

        assert!(GameLogRecord::get_toi_as_secs(":100:1").is_err()); // malformed
        assert!(GameLogRecord::get_toi_as_secs(":1").is_err()); // must at least have minutes
        assert!(GameLogRecord::get_toi_as_secs("1").is_err()); // must at least have minutes
    }

}