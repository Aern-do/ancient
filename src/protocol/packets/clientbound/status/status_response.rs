use derive_macro::Writeable;

use crate::packet;

#[derive(Debug, Clone, Writeable)]
pub struct StatusResponse {
    pub response: String,
}
packet!(0x0 -> StatusResponse);
