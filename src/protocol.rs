use crate::protocol::error::ProtocolError;
use std::str::from_utf8;

pub mod error;

const TAG_PRODUCE: u8 = 1;
const TAG_FETCH: u8 = 2;

pub enum Request {
    Produce(ProduceRequest),
    Fetch(FetchRequest),
}

impl Request {
    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        match self {
            Request::Produce(req) => {
                buffer.push(TAG_PRODUCE);
                req.encode(&mut buffer)
            }
            Request::Fetch(req) => {
                buffer.push(TAG_FETCH);
                req.encode(&mut buffer)
            }
        }
        buffer
    }

    pub fn decode(src: &[u8]) -> Result<Self, ProtocolError> {
        let (&tag, rest) = src.split_first().ok_or(ProtocolError::EmptyInput)?;
        match tag {
            TAG_PRODUCE => {
                let req = ProduceRequest::decode(rest)?;
                Ok(Request::Produce(req))
            }
            TAG_FETCH => {
                let req = FetchRequest::decode(rest)?;
                Ok(Request::Fetch(req))
            }
            tag => return Err(ProtocolError::InvalidRequestType(tag)),
        }
    }
}

pub enum Response {
    Produce(ProduceResponse),
    Fetch(FetchResponse),
    Error(ProtocolError),
}

impl Response {}

pub struct ProduceRequest {
    pub topic: String,
    pub partition: u32,
    pub payload: Vec<u8>,
}

impl ProduceRequest {
    pub fn encode(&self, buffer: &mut Vec<u8>) {
        let topic_len = self.topic.len() as u16;
        buffer.extend_from_slice(&topic_len.to_be_bytes());
        buffer.extend_from_slice(self.topic.as_bytes());

        buffer.extend_from_slice(&self.partition.to_be_bytes());

        let payload_len = self.payload.len() as u32;
        buffer.extend_from_slice(&payload_len.to_be_bytes());
        buffer.extend_from_slice(&self.payload);
    }

    pub fn decode(src: &[u8]) -> Result<Self, ProtocolError> {
        let mut rest = src;
        if src.is_empty() {
            return Err(ProtocolError::EmptyInput);
        }

        let (head, r) = rest
            .split_first_chunk::<2>()
            .ok_or(ProtocolError::IncompleteParse)?;
        let topic_len = u16::from_be_bytes(*head) as usize;
        rest = r;

        let (head, r) = rest
            .split_at_checked(topic_len)
            .ok_or(ProtocolError::IncompleteParse)?;
        let topic = from_utf8(head)
            .map_err(|_| ProtocolError::InvalidUtf8)?
            .to_string();
        rest = r;

        let (head, r) = rest
            .split_first_chunk::<4>()
            .ok_or(ProtocolError::IncompleteParse)?;
        let partition = u32::from_be_bytes(*head);
        rest = r;

        let (head, r) = rest
            .split_first_chunk::<4>()
            .ok_or(ProtocolError::IncompleteParse)?;
        let payload_len = u32::from_be_bytes(*head) as usize;
        rest = r;

        let (head, _) = rest
            .split_at_checked(payload_len)
            .ok_or(ProtocolError::IncompleteParse)?;
        let payload = head.to_vec();

        Ok(ProduceRequest {
            topic,
            partition: partition,
            payload: payload,
        })
    }
}

pub struct ProduceResponse {
    pub offset: u64,
}

impl ProduceResponse {
    pub fn encode(&self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(&self.offset.to_be_bytes());
    }
}

pub struct FetchRequest {
    pub offset: u64,
}

impl FetchRequest {
    pub fn encode(&self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(&self.offset.to_be_bytes());
    }

    pub fn decode(src: &[u8]) -> Result<Self, ProtocolError> {
        todo!()
    }
}

pub struct FetchResponse {
    pub topic: String,
    pub partition: u32,
    pub payload: Vec<u8>,
}

impl FetchResponse {
    pub fn encode(&self, buffer: &mut Vec<u8>) {
        //buffer.extend_from_slice(&self.offset.to_be_bytes());
    }
}
