use std::{
    net::UdpSocket,
    sync::mpsc::{
        Sender,
        Receiver,
    },
};
use crate::utils::networking::packet::{Packet,MTU};

pub struct NetworkHandler{
    pub socket: UdpSocket,
    pub send_rx: Receiver<Packet>,
    pub recv_tx: Sender<Packet>
}

impl NetworkHandler{
    pub fn new(socket: UdpSocket, send_rx: Receiver<Packet>, recv_tx: Sender<Packet>) -> NetworkHandler{
        NetworkHandler{
            socket,
            send_rx,
            recv_tx,
        }
    }

    pub fn recv(&self)->Packet{
        let mut buf = [0; MTU];
        let (amt, src) = self.socket.recv_from(&mut buf).unwrap();
        Packet{
            addr: src,
            payload: buf,
        }
    }

    pub fn send(&self,packet: Packet){
        self.socket.send_to(&packet.payload, packet.addr).unwrap();
    }
}

