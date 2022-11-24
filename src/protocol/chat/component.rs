use serde::{Deserialize, Serialize};

use super::{click_event::ClickEvent, color::Color, font::Font};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Component {
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub bold: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub italic: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub underlined: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub strikethrough: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub obfuscated: bool,
    pub font: Option<Font>,
    pub color: Option<Color>,
    pub insertion: Option<String>,
    pub click_event: Option<ClickEvent>,
    #[serde(flatten)]
    pub text: Option<TextComponent>,
    #[serde(flatten)]
    pub translate: Option<TranslationComponent>,
    #[serde(flatten)]
    pub keybind: Option<KeyBindComponent>,
    pub extra: Option<Vec<Component>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TextComponent {
    pub text: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TranslationComponent {
    pub translate: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KeyBindComponent {
    pub keybind: String,
}
