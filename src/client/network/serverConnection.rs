use std::sync::mpsc::{Sender, Receiver};
use std::net::SocketAddr;
use std::time::SystemTime;

use crate::utils::networking::packet::*;
use crate::utils::networking::protocol::*;

pub struct ServerConnection {
    pub connected: bool,
    pub addr: SocketAddr,
    packet_rx: Receiver<Packet>,
    packet_tx: Sender<Packet>,
    last_sent: SystemTime,
    last_received: SystemTime
}

impl ServerConnection {
    pub fn new(server: SocketAddr, packet_rx: Receiver<Packet>, packet_tx: Sender<Packet>) -> ServerConnection {
        ServerConnection {
            connected: false,
            addr: server,
            packet_rx: packet_rx,
            packet_tx: packet_tx,
            last_sent: SystemTime::UNIX_EPOCH,
            last_received: SystemTime::UNIX_EPOCH,
        }
    }

    pub fn connect(&mut self) {
        if !self.connected {
            let mut payload = EMPTY_PACKET;
            payload[0] = OPCODE_LOGIN;
            self.send(payload);
        }
    }

    fn send(&mut self, payload: [u8; PACKET_MAX_LENGTH]) {
        let packet = Packet {addr: self.addr, payload: payload};
        self.packet_tx.send(packet).unwrap();
        self.last_sent = SystemTime::now();
    }

    fn ping(&mut self) {
        let mut payload = EMPTY_PACKET;
        payload[0] = OPCODE_PING;
        self.send(payload);
    }    
}