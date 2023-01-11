use derive_macro::{Readable, Writeable};

use crate::{
    packet,
    protocol::{chat::component::Component, json::Json},
};

#[derive(Clone, Debug, Readable, Writeable)]
pub struct Disconnect {
    component: Json<Component>,
}
packet!(0x0 -> Disconnect);
