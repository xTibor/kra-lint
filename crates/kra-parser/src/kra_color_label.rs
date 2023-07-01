use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, PartialEq, Deserialize, Serialize, Display, EnumString)]
#[serde(rename_all = "snake_case")]
pub enum KraColorLabel {
    #[strum(serialize = "0", to_string = "none")]
    None,

    #[strum(serialize = "1", to_string = "blue")]
    Blue,

    #[strum(serialize = "2", to_string = "green")]
    Green,

    #[strum(serialize = "3", to_string = "yellow")]
    Yellow,

    #[strum(serialize = "4", to_string = "orange")]
    Orange,

    #[strum(serialize = "5", to_string = "brown")]
    Brown,

    #[strum(serialize = "6", to_string = "red")]
    Red,

    #[strum(serialize = "7", to_string = "purple")]
    Purple,

    #[strum(serialize = "8", to_string = "black")]
    Black,
}
