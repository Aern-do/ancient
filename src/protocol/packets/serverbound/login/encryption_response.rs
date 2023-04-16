use derive_macro::{Decode, Encode};
use num_bigint::BigInt;
use rsa::{pkcs8::EncodePublicKey, PaddingScheme};
use sha1::{Digest, Sha1};
use uuid::Uuid;

use crate::{
    connection::Connection,
    error::Error,
    protocol::{
        auth_response::AuthResponse,
        packets::clientbound::login::login_success::{self, LoginSuccess},
        varint::VarInt,
        vec::PrefixedVec,
        Processable,
    },
    RSA_KEYPAIR,
};

#[derive(Debug, Clone, Decode, Encode)]
pub struct EncryptionResponse {
    #[inner(PrefixedVec<_, VarInt>)]
    shared_secret: Vec<u8>,
    #[inner(PrefixedVec<_, VarInt>)]
    verify_token: Vec<u8>,
}
impl Processable for EncryptionResponse {
    fn process(self, connection: &mut Connection) -> Result<(), Error> {
        let shared_secret =
            RSA_KEYPAIR.decrypt(PaddingScheme::PKCS1v15Encrypt, &self.shared_secret)?;
        let client_verify_token =
            RSA_KEYPAIR.decrypt(PaddingScheme::PKCS1v15Encrypt, &self.verify_token)?;
        if client_verify_token != connection.verify_token {
            panic!("Verify tokens dosen't match");
        }
        let secret = shared_secret.try_into().expect("Invalid shared secret");
        connection.enable_encryption(secret);
        let response = auth(secret, connection.user.as_ref().unwrap())?;
        let properties = response
            .properties
            .into_iter()
            .map(|property| login_success::Property {
                name: property.name,
                value: property.value,
                signature: Some(property.signature),
            })
            .collect::<Vec<_>>();
        let login_success = LoginSuccess {
            uuid: Uuid::parse_str(&response.id)?,
            username: connection.user.clone().expect("Missing user"),
            properties,
        };
        connection.write_packet(login_success)?;
        Ok(())
    }
}
fn auth(secret: [u8; 16], username: &str) -> Result<AuthResponse, Error> {
    let response = ureq::get(&format!(
        "https://sessionserver.mojang.com/session/minecraft/hasJoined?username={}&serverId={}",
        username,
        compute_hash(&secret)
    ))
    .call()
    .map_err(|_| Error::Auth)?
    .into_json()?;
    Ok(response)
}
// https://gist.github.com/RoccoDev/8fa130f1946f89702f799f89b8469bc9
fn compute_hash(secret: &[u8; 16]) -> String {
    let mut hasher = Sha1::default();
    hasher.update(b"");
    hasher.update(secret);
    hasher.update(RSA_KEYPAIR.to_public_key().to_public_key_der().unwrap());
    BigInt::from_signed_bytes_be(hasher.finalize().as_slice()).to_str_radix(16)
}
