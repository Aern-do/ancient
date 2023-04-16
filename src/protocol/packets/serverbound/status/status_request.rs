use derive_macro::Decode;

use crate::{
    connection::Connection,
    error::Error,
    protocol::{
        packets::clientbound::status::status_response::StatusResponse, Description, Players,
        Processable, Status, Version,
    },
};

#[derive(Clone, Debug, Decode)]
pub struct StatusRequest {}
impl Processable for StatusRequest {
    fn process(self, connection: &mut Connection) -> Result<(), Error> {
        let status = Status {
            version: Version {
                name: String::from("1.19.3"),
                protocol: 761,
            },
            players: Players {
                max: 2022,
                online: 0,
                sample: vec![],
            },
            description: Description {
                text: String::from("Minecraft server running on Rust"),
            },
            favicon: None,
            previews_chat: false,
            enforces_secure_chat: false,
        };
        let response = serde_json::to_string_pretty(&status).unwrap();
        connection.write_packet(StatusResponse { response })?;
        Ok(())
    }
}
