use std::net::SocketAddr;

pub const MTU: usize = 1472;
pub const EMPTY_PACKET: [u8; MTU] = [0; MTU];




pub struct Packet {
    pub addr: SocketAddr,
    pub payload: [u8; MTU]
}