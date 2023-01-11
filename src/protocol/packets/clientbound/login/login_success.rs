use derive_macro::Writeable;
use uuid::Uuid;

use crate::{
    packet,
    protocol::{varint::VarInt, vec::PrefixedVec},
};

#[derive(Debug, Clone, Writeable)]
pub struct LoginSuccess {
    pub uuid: Uuid,
    pub username: String,
    #[inner(PrefixedVec<_, VarInt>)]
    pub properties: Vec<Property>,
}
packet!(0x02 -> LoginSuccess);
#[derive(Debug, Clone, Writeable)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}
