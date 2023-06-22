use std::sync::mpsc::{Sender, Receiver};
use std::net::SocketAddr;
use std::time::SystemTime;
use crate::utils::networking::packet::Packet;
use crate::utils::networking::protocol::*;

pub struct Peer {
    username: String,
    addr: SocketAddr,
    packet_rx: Receiver<Packet>,
    packet_tx: Sender<Packet>,
    last_sent: SystemTime,
    last_received: SystemTime
}

impl Peer {
    pub fn connect(username: String, addr: SocketAddr, packet_rx: Receiver<Packet>, packet_tx: Sender<Packet>) -> Peer {
        // connection logic
        Peer {
            username: username,
            addr: addr,
            packet_rx: packet_rx,
            packet_tx: packet_tx,
            last_sent: SystemTime::now(),
            last_received: SystemTime::now(),
        }
    }

    pub fn send(&mut self, payload: [u8; PACKET_MAX_LENGTH]) {
        let packet = Packet {addr: self.addr, payload: payload};
        self.packet_tx.send(packet).unwrap();
    }
}