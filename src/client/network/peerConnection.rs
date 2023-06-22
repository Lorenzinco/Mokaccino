use std::sync::mpsc::{Sender, Receiver};
use std::net::SocketAddr;
use std::time::SystemTime;
use crate::utils::networking::packet::*;
use crate::utils::networking::protocol::*;

pub struct PeerConnection {
    pub connected: bool,
    pub username: Option<String>,
    pub addr: Option<SocketAddr>,
    packet_rx: Receiver<Packet>,
    packet_tx: Sender<Packet>,
    last_sent: SystemTime,
    last_received: SystemTime
}

impl PeerConnection {
    pub fn new(packet_rx: Receiver<Packet>, packet_tx: Sender<Packet>) -> PeerConnection {
        PeerConnection {
            connected: false,
            username: None,
            addr: None,
            packet_rx: packet_rx,
            packet_tx: packet_tx,
            last_sent: SystemTime::UNIX_EPOCH,
            last_received: SystemTime::UNIX_EPOCH,
        }
    }

    fn send(&mut self, payload: [u8; PACKET_MAX_LENGTH]) {
        if self.connected {
            let packet = Packet {addr: self.addr.unwrap(), payload: payload};
            self.packet_tx.send(packet).unwrap();
            self.last_sent = SystemTime::now();
        }
    }
}