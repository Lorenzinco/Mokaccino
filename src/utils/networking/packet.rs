use std::net::SocketAddr;
use crate::utils::networking::protocol::{*};

pub const EMPTY_PACKET: [u8; PACKET_MAX_LENGTH] = [0; PACKET_MAX_LENGTH];

pub struct Packet {
    pub addr: SocketAddr,
    pub payload: [u8; PACKET_MAX_LENGTH]
}