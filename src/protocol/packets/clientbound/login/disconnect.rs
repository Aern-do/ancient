use derive_macro::{Decode, Encode};

use crate::{
    packet,
    protocol::{chat::component::Component, json::Json},
};

#[derive(Clone, Debug, Decode, Encode)]
pub struct Disconnect {
    component: Json<Component>,
}
packet!(0x0 -> Disconnect);
