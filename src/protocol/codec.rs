use std::io::Cursor;

use prost::Message;
use tokio_util::codec::{Decoder, Encoder};

use super::rusty::belt::{Request, Response};

pub struct RequestCodec {}

impl Encoder<Request> for RequestCodec {
    type Error = std::io::Error;
    fn encode(
        &mut self,
        item: Request,
        dst: &mut prost::bytes::BytesMut,
    ) -> Result<(), Self::Error> {
        item.encode_length_delimited(dst)
            .map_err(|e| std::io::Error::other(e))
    }
}

impl Decoder for RequestCodec {
    type Error = std::io::Error;
    type Item = Request;

    fn decode(
        &mut self,
        src: &mut prost::bytes::BytesMut,
    ) -> Result<Option<Self::Item>, Self::Error> {
        let len = src.len();
        if len == 0 {
            // there are no bytes to consume, stop querying the buffer
            return Ok(None);
        }
        let bytes = Cursor::new(src.clone().to_vec());
        Request::decode_length_delimited(bytes)
            .map(|v| Some(v))
            .map_err(|e| std::io::Error::other(e))
    }
}

pub struct ResponseCodec {}

impl Encoder<Response> for ResponseCodec {
    type Error = std::io::Error;
    fn encode(
        &mut self,
        item: Response,
        dst: &mut prost::bytes::BytesMut,
    ) -> Result<(), Self::Error> {
        item.encode_length_delimited(dst)
            .map_err(|e| std::io::Error::other(e))
    }
}

impl Decoder for ResponseCodec {
    type Error = std::io::Error;
    type Item = Response;
    fn decode(
        &mut self,
        src: &mut prost::bytes::BytesMut,
    ) -> Result<Option<Self::Item>, Self::Error> {
        let len = src.len();
        if len == 0 {
            // there are no bytes to consume, stop querying the buffer
            return Ok(None);
        }
        let bytes = Cursor::new(src.clone().to_vec());
        Response::decode_length_delimited(bytes)
            .map(|v| Some(v))
            .map_err(|e| std::io::Error::other(e))
    }
}
