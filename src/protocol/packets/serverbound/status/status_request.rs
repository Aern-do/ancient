use std::io::{Read, Write};

use derive_macro::Readable;

use crate::{
    connection::Connection,
    error::Error,
    protocol::{
        packets::clientbound::status::status_response::StatusResponse, Description, Players,
        Processable, Status, Version,
    },
};

#[derive(Clone, Debug, Readable)]
pub struct StatusRequest {}
impl Processable for StatusRequest {
    fn process<S: Read + Write>(
        self,
        stream: &mut S,
        connection: &mut Connection,
    ) -> Result<(), Error> {
        let status = Status {
            version: Version {
                name: String::from("1.19.2"),
                protocol: 760,
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
        connection.write_packet(stream, StatusResponse { response })?;
        Ok(())
    }
}
