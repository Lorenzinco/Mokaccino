use std::sync::mpsc::{Sender, Receiver};
use std::net::SocketAddr;
use std::time::SystemTime;

use crate::utils::networking::packet::Packet;
use crate::utils::networking::protocol::*;

pub struct Server {
    pub addr: SocketAddr,
    packet_rx: Receiver<Packet>,
    packet_tx: Sender<Packet>,
    last_sent: SystemTime,
    last_received: SystemTime
}

impl Server {
    pub fn connect(server: SocketAddr, packet_rx: Receiver<Packet>, packet_tx: Sender<Packet>) -> Server {
        // connection logic
        Server {
            addr: server,
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

    pub fn ping(&mut self) {
        self.send([OPCODE_PING; PACKET_MAX_LENGTH]); // ...meh
    }
}