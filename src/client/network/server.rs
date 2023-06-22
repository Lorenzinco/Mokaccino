use std::net::{IpAddr};
use std::sync::mpsc::{Sender, Receiver};

#[path = "../../utils/mod.rs"]
mod utils;
use utils::networking::socket::Packet;

pub struct Server {
    last_ping: u64,
    ip: IpAddr,
    port: u16,
    packet_rx: Receiver<Packet>,
    packet_tx: Sender<Packet>
}

impl Server {
    pub fn connect(server: IpAddr) -> Server {
        // ...
    }
}