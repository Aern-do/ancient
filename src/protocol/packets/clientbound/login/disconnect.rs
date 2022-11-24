use derive_macro::{Readable, Writeable};

use crate::{protocol::{json::Json, chat::component::Component}, packet};

#[derive(Clone, Debug, Readable, Writeable)]
pub struct Disconnect {
    component: Json<Component>
}
packet!(0x0 -> Disconnect);