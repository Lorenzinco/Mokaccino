use log;
use std::io::ErrorKind;
use std::net::{UdpSocket, SocketAddr};
use std::sync::mpsc::{Sender, Receiver};
use crate::utils::networking::protocol;
use crate::utils::networking::packet::Packet;
use crate::utils::terminal::cli;

pub struct Socket {
    socket: UdpSocket,
    server: SocketAddr
}

impl Socket {
    //constructor
    pub fn new(server: SocketAddr) -> Socket {
        let socket = UdpSocket::bind("0.0.0.0:0").expect(&format!("Couldn't bind UDP socket"));
        socket.set_nonblocking(true).expect("Couldn't set UDP socket in non-blocking mode");
        Socket {
            socket: socket,
            server: server
        }
    }

    pub fn send(&self, outgoing_rx: Receiver<Packet>) {
        let packet = outgoing_rx.recv().unwrap();
        log::debug!("Sending packet to {}", packet.addr);
        self.socket.send_to(&packet.payload, packet.addr).expect("Couldn't send UDP packet");
    }

    pub fn receive(&self, server_tx: Sender<Packet>, peer_tx: Sender<Packet>) {
        let mut buffer: [u8; protocol::PACKET_MAX_LENGTH] = [0; protocol::PACKET_MAX_LENGTH];
        let result = self.socket.recv_from(&mut buffer);
        match result {
            Ok((number_of_bytes, source)) => {
                log::debug!("Received {} bytes from {}", number_of_bytes, source);
                let packet = Packet {
                    addr: source,
                    payload: buffer
                };
                if source == self.server {
                    server_tx.send(packet).expect("Couldn't queue UDP packet while communicating with server");
                } else {
                    peer_tx.send(packet).expect("Couldn't queue UDP packet while communicating with peer");
                }
            },
            Err(ref err) if err.kind() != ErrorKind::WouldBlock => {
                cli::error(format!("Error while receiving from UDP socket: {}", err).as_str());
                
            },
            _ => {} // no incoming packets
        };
    }
}