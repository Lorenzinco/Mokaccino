use std::net::SocketAddr;
use std::time::SystemTime;

pub struct Peer {
    pub username: String,
    pub addr: SocketAddr,
    pub last_ping: SystemTime,
}

impl Peer {
    
}