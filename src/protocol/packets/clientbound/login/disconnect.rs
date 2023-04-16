use derive_macro::{Decode, Encode};
use serde::{Serialize, Deserialize};

use crate::{
    packet,
    protocol::{json::Json},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DisconnectMessage {
    pub text: String
}
#[derive(Clone, Debug, Decode, Encode)]
pub struct Disconnect {
    #[custom(ty: Json<_>; decode: into_inner, encode: from_inner)]
    pub component: DisconnectMessage,
}
packet!(0x0 -> Disconnect);
