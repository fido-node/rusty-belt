use std::path::PathBuf;

use crate::protocol::{
    codec::{RequestCodec, ResponseCodec},
    rusty::belt,
};
use futures::{SinkExt, StreamExt};
use log::error;
use tokio::net::{
    unix::{ReadHalf, WriteHalf},
    UnixStream,
};
use tokio_util::codec::{FramedRead, FramedWrite};

pub struct CliClient {
    addr: String,
}

impl CliClient {
    pub fn new(addr: String) -> CliClient {
        CliClient { addr }
    }

    pub async fn make_request(self, request: belt::Request) -> Result<belt::Response, ()> {
        if let Ok(mut connection) = UnixStream::connect(self.addr).await {
            let (read_stream, write_stream) = connection.split();

            let reader = FramedRead::new(read_stream, ResponseCodec {});
            let writer = FramedWrite::new(write_stream, RequestCodec {});

            let connection = ClientConnection { reader, writer };
            connection.make_request(request).await
        } else {
            Err(())
        }
    }
}

struct ClientConnection<'a> {
    writer: FramedWrite<WriteHalf<'a>, RequestCodec>,
    reader: FramedRead<ReadHalf<'a>, ResponseCodec>,
}

impl ClientConnection<'_> {
    pub async fn make_request(mut self, request: belt::Request) -> Result<belt::Response, ()> {
        if let Ok(_ignore) = self.writer.send(request).await {
            if let Some(msg) = self.reader.next().await {
                msg.map_err(|err| {
                    error!("{:?}", err);
                    ()
                })
            } else {
                error!("can't read server response");
                Err(())
            }
        } else {
            error!("can't send to server");
            Err(())
        }
    }
}
