use std::net::SocketAddr;
use crate::utils::networking::protocol::{*};

pub struct Packet {
    pub addr: SocketAddr,
    pub payload: [u8; PACKET_MAX_LENGTH]
}