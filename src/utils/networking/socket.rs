use log::debug;
use std::net::{IpAddr, UdpSocket};
use std::sync::mpsc;

#[path = "./constants.rs"]
mod constants;

pub struct Packet {
    ip: IpAddr,
    port: u16,
    payload: [u8; PACKET_MAX_LENGTH]
}

pub struct Socket {
    socket: UdpSocket,
    server: IpAddr,
    outgoing_rx: Receiver<Packet>,
    server_tx: Sender<Packet>,
    peer_tx: Sender<Packet>
}

impl Socket {
    //constructor
    pub fn new(port: u16, server: IpAddr, outgoing_rx: Receiver<>, server_tx: Sender<>, peer_tx: Sender<>) -> StreamHandler {
        let addr = format!("0.0.0.0:{}", port);
        debug!("Creating a new UDP socket at {}", addr);
        StreamHandler {
            socket: UdpSocket::bind(addr).expect(format!("Couldn't bind UDP socket to {}", addr).as_str())
        }
    }

    pub fn send(&self, packet: Packet) {
        debug!("Sending packet to {}:{}", packet.ip, packet.port);
        self.socket
            .send_to(packet.payload.as_bytes(), (packet.ip, packet.port))
            .expect("Couldn't send UDP packet");
    }

    pub fn receive(&self) {
        let mut buffer = [u8; PACKET_MAX_LENGTH];
        let (numbytes, source) = self
            .socket
            .recv_from(&mut buffer)
            .expect("Couldn't receive UDP packet");
        debug!("Received {} bytes from {}", numbytes, source);
        packet = Packet {
            ip: source.ip(),
            port: source.port(),
            payload: buffer
        };
        if (packet.ip == self.server) {
            self.server_tx.send(packet);
        } else {
            self.peer_tx.send(packet);
        }
    }
}
