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
    fn process(self, connection: &mut Connection) -> Result<(), Error> {
        connection.write_packet(PingResponse {
            payload: self.payload,
        })
    }
}
