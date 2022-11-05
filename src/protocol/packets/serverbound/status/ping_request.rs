use std::io::{Read, Write};

use derive_macro::Readable;

use crate::{
    connection::Connection,
    error::Error,
    protocol::{packets::clientbound::status::ping_response::PingResponse, Processable},
};

#[derive(Debug, Clone, Readable)]
pub struct PingRequest {
    pub payload: i64,
}

impl Processable for PingRequest {
    fn process<S: Read + Write>(
        self,
        stream: &mut S,
        connection: &mut Connection,
    ) -> Result<(), Error> {
        connection.write_packet(
            stream,
            PingResponse {
                payload: self.payload,
            },
        )
    }
}
