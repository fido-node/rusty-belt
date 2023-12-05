use std::sync::Arc;

use crate::{
    protocol::codec::{RequestCodec, ResponseCodec},
    state::State,
};
use futures::{SinkExt, StreamExt};
use tokio::net::{
    unix::{ReadHalf, WriteHalf},
    UnixListener,
};
use tokio_util::codec::{FramedRead, FramedWrite};

pub struct Server {
    state: Arc<State>,
    addr: String,
}

impl Server {
    pub fn new(state: Arc<State>, addr: String) -> Server {
        Server { state, addr }
    }

    pub async fn run(&self) -> () {
        let addr = &self.addr;
        tokio::fs::remove_file(addr).await.ok();
        let listener = UnixListener::bind(addr).unwrap();

        loop {
            match listener.accept().await {
                Ok((mut stream, _addr)) => {
                    let (read_stream, write_stream) = stream.split();

                    let reader = FramedRead::new(read_stream, RequestCodec {});
                    let writer = FramedWrite::new(write_stream, ResponseCodec {});

                    let connection = ServerConnection {
                        writer,
                        reader,
                        state: self.state.as_ref(),
                    };
                    connection.process_message().await
                }
                Err(_e) => { /* connection failed */ }
            }
        }
    }
}

pub struct ServerConnection<'a> {
    writer: FramedWrite<WriteHalf<'a>, ResponseCodec>,
    reader: FramedRead<ReadHalf<'a>, RequestCodec>,
    state: &'a State,
}

impl ServerConnection<'_> {
    pub async fn process_message(mut self) -> () {
        if let Some(Ok(message)) = self.reader.next().await {
            log::debug!("Incoming request: {:?}", message);
            if let Ok(response) = self.state.fetch_info(message) {
                            log::debug!("Going to send response: {:?}", response);
                if let Ok(_) = self.writer.send(response).await {
                    ()
                } else {
                    ()
                }
            } else {
                ()
            }
        } else {
            ()
        }
    }
}
