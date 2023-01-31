use rusqlite::{self, ToSql, types::ToSqlOutput};
use serde::{Deserialize, Serialize,};
use strum::{EnumString, Display};

// Position
// An enum representing all the possible positions a player could play in a game.
// If the EP-API does not give a position, 'None' is the default. 
#[derive(PartialEq, PartialOrd, Clone, Debug, Deserialize, Serialize, EnumString, Display)]
pub enum Position {
    #[strum(serialize = "Forward")]
    #[serde(alias = "FORWARD", alias = "F")]
    Forward,

    #[strum(serialize = "Defense")]
    #[serde(alias = "DEFENSE", alias = "D")]
    Defense,

    #[strum(serialize = "Goalkeeper")]
    #[serde(alias = "GOALKEEPER", alias = "G")]
    Goalkeeper,

    #[strum(serialize = "Null")]
    #[serde(alias = "NULL", alias = "null", alias = "Null", alias = "N")]
    None,
}

impl ToSql for Position {
    #[inline]
    fn to_sql(&self) ->  rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.to_string()))
    }

}