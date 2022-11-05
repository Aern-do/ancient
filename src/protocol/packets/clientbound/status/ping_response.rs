use derive_macro::Writeable;

use crate::packet;

#[derive(Clone, Debug, Writeable)]
pub struct PingResponse {
    pub payload: i64,
}
packet!(0x1 -> PingResponse);
