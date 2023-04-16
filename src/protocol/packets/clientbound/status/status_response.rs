use derive_macro::Encode;

use crate::packet;

#[derive(Debug, Clone, Encode)]
pub struct StatusResponse {
    pub response: String,
}
packet!(0x0 -> StatusResponse);
