use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(try_from = "String")]
#[serde(untagged)]
pub enum Color {
    Black,
    DarkBlue,
    DarkGreen,
    DarkAqua,
    DarkRed,
    DarkPurple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    Green,
    Aqua,
    Red,
    LightPurple,
    Yellow,
    White,
    #[serde(serialize_with = "self::seralize")]
    Custom(u32),
    Reset,
}
fn seralize<S: Serializer>(hex: &u32, seralizer: S) -> Result<S::Ok, S::Error> {
    seralizer.serialize_str(&format!("#{:X}", hex))
}
impl TryFrom<String> for Color {
    type Error = &'static str;
    fn try_from(string: String) -> Result<Self, Self::Error> {
        match string.as_ref() {
            "black" => Ok(Self::Black),
            "dark_blue" => Ok(Self::DarkBlue),
            "dark_green" => Ok(Self::DarkGreen),
            "dark_aqua" => Ok(Self::DarkAqua),
            "dark_red" => Ok(Self::DarkRed),
            "dark_purple" => Ok(Self::DarkPurple),
            "gold" => Ok(Self::Gold),
            "gray" => Ok(Self::Gray),
            "dark_gray" => Ok(Self::DarkGray),
            "blue" => Ok(Self::Blue),
            "green" => Ok(Self::Green),
            "red" => Ok(Self::Red),
            "light_purple" => Ok(Self::LightPurple),
            "yellow" => Ok(Self::Yellow),
            "white" => Ok(Self::White),
            "reset" => Ok(Self::Reset),
            string if string.starts_with('#') => {
                Ok(Self::Custom(u32::from_str_radix(&string[1..], 16).unwrap()))
            }
            _ => Err("Expected minecraft or hex color"),
        }
    }
}
