use derive_macro::{Readable, Writeable};
use log::info;
use rsa::pkcs8::EncodePublicKey;
use uuid::Uuid;

use crate::{
    connection::Connection,
    error::Error,
    packet,
    protocol::{packets::clientbound::login::encryption_request::EncryptionRequest, Processable},
    RSA_KEYPAIR,
};

#[derive(Debug, Clone, Writeable, Readable)]
pub struct LoginStart {
    name: String,
    player_uuid: Option<Uuid>,
}
packet!(0x0 -> LoginStart);

impl Processable for LoginStart {
    fn process(self, connection: &mut Connection) -> Result<(), Error> {
        info!("{} connecting to server", &self.name);
        connection.user = Some(self.name);
        connection.write_packet(EncryptionRequest {
            server_id: String::from(""),
            public_key: RSA_KEYPAIR
                .to_public_key()
                .to_public_key_der()
                .unwrap()
                .as_bytes()
                .to_owned(),
            verify_token: connection.verify_token,
        })?;
        Ok(())
    }
}
