use serde::{Deserialize, Serialize,};
use strum::EnumString;


#[derive(PartialEq, PartialOrd, Clone, Debug, Deserialize, Serialize, EnumString)]
pub enum Position {
    #[serde(alias = "FORWARD", alias = "F")]
    Forward,

    #[serde(alias = "DEFENSE", alias = "D")]
    Defense,

    #[serde(alias = "GOALKEEPER", alias = "G")]
    Goalkeeper,

    #[serde(alias = "NULL", alias = "null", alias = "Null", alias = "N")]
    None,
}
