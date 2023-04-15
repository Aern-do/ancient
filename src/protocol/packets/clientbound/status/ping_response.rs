use derive_macro::Encode;

use crate::packet;

#[derive(Clone, Debug, Encode)]
pub struct PingResponse {
    pub payload: i64,
}
packet!(0x1 -> PingResponse);
