use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClickEvent {
    pub open_url: Option<String>,
    pub open_file: Option<String>,
    pub run_command: Option<String>,
    pub suggest_command: Option<String>,
    pub change_page: Option<ChangeBookPage>,
    pub copy_to_clipboard: Option<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChangeBookPage {
    pub value: usize,
}
