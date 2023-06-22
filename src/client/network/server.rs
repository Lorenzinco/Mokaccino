use std::sync::mpsc::{Sender, Receiver};
use std::net::SocketAddr;
use std::time::SystemTime;

use crate::utils::networking::socket::Packet;

pub struct Server {
    last_ping: SystemTime,
    addr: SocketAddr,
    packet_rx: Receiver<Packet>,
    packet_tx: Sender<Packet>
}

impl Server {
    pub fn connect(server: SocketAddr, packet_rx: Receiver<Packet>, packet_tx: Sender<Packet>) -> Server {
        Server {
            last_ping: SystemTime::now(),
            addr: server,
            packet_rx: packet_rx,
            packet_tx: packet_tx
        }
    }
}