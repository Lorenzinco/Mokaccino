use std::sync::mpsc::{Sender, Receiver};
use std::net::SocketAddr;
use std::time::SystemTime;

use crate::utils::networking::socket::{Packet, Socket};
use crate::utils::networking::protocol::{*};

pub struct Server {
    last_ping: SystemTime,
    addr: SocketAddr,
    packet_rx: Receiver<Packet>,
    packet_tx: Sender<Packet>
}

impl Server {
    pub fn connect(server: SocketAddr, packet_rx: Receiver<Packet>, packet_tx: Sender<Packet>) -> Server {
        // connection logic
        Server {
            last_ping: SystemTime::now(),
            addr: server,
            packet_rx: packet_rx,
            packet_tx: packet_tx
        }
    }

    pub fn send(&mut self, packet: Packet) {
        // send logic
        self.packet_tx.send(packet).unwrap();
    }

    pub fn ping(&mut self) {
        // ping logic
        let packet: Packet = Packet{addr : self.addr,payload : [OPCODE_PING;PACKET_MAX_LENGTH]};
        self.send(packet);   
    }
}