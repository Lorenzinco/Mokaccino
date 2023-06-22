use log;
use std::net::{UdpSocket, SocketAddr};
use std::sync::mpsc::{Sender, Receiver};

use crate::utils::networking::protocol;

pub struct Packet {
    pub addr: SocketAddr,
    pub payload: [u8; protocol::PACKET_MAX_LENGTH]
}

pub struct Socket {
    socket: UdpSocket,
    server: SocketAddr,
    outgoing_rx: Receiver<Packet>,
    server_tx: Sender<Packet>,
    peer_tx: Sender<Packet>
}

impl Socket {
    //constructor
    pub fn new(port: u16, server: SocketAddr, outgoing_rx: Receiver<Packet>, server_tx: Sender<Packet>, peer_tx: Sender<Packet>) -> Socket {
        let addr = format!("0.0.0.0:{}", port);
        log::debug!("Creating a new UDP socket at {}", &addr);
        Socket {
            socket: UdpSocket::bind(&addr).expect(&format!("Couldn't bind UDP socket to {}", &addr)),
            server: server,
            outgoing_rx: outgoing_rx,
            server_tx: server_tx,
            peer_tx: peer_tx
        }
    }

    pub fn send(&self, packet: Packet) {
        log::debug!("Sending packet to {}", packet.addr);
        self.socket
            .send_to(&packet.payload, packet.addr)
            .expect("Couldn't send UDP packet");
    }

    pub fn receive(&self) {
        let mut buffer: [u8; protocol::PACKET_MAX_LENGTH] = [0; protocol::PACKET_MAX_LENGTH];
        let (numbytes, source) = self
            .socket
            .recv_from(&mut buffer)
            .expect("Couldn't receive UDP packet");
        log::debug!("Received {} bytes from {}", numbytes, source);
        let packet = Packet {
            addr: source,
            payload: buffer
        };
        if source == self.server {
            self.server_tx.send(packet).expect("Couldn't queue UDP packet while communicating with server");
        } else {
            self.peer_tx.send(packet).expect("Couldn't queue UDP packet while communicating with peer");
        }
    }
}
