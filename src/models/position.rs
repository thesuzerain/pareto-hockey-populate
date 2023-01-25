use serde::{Deserialize, Serialize,};
use strum::{EnumString, Display};


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
