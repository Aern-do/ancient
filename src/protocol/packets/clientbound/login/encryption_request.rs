use derive_macro::{Decode, Encode};

use crate::{
    packet,
    protocol::{array::PrefixedArray, varint::VarInt, vec::PrefixedVec},
};

#[derive(Debug, Clone, Decode, Encode)]
pub struct EncryptionRequest {
    pub server_id: String,
    #[inner(PrefixedVec<u8, VarInt>)]
    pub public_key: Vec<u8>,
    #[inner(PrefixedArray<VarInt, _, 4>)]
    pub verify_token: [u8; 4],
}
packet!(0x1 -> EncryptionRequest);
