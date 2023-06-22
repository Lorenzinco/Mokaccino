use std::net::SocketAddr;
use std::time::SystemTime;

pub struct Peer {
    last_ping: SystemTime,
    addr: SocketAddr,
    username: String,
}

impl Peer {
    
}