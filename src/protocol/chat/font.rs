use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Font {
    #[serde(rename = "minecraft::uniform")]
    Unicode,
    #[serde(rename = "minecraft::alt")]
    Alternative,
    #[serde(rename = "minecraft::default")]
    Default,
}
