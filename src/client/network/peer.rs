use std::net::SocketAddr;
use std::time::SystemTime;

pub struct Peer {
    username: String,
    addr: SocketAddr,
    last_sent: SystemTime,
    last_received: SystemTime
}

impl Peer {
    
}