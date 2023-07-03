use std::{
    net::UdpSocket,
};
use crate::utils::networking::packet::{Packet,MTU};
pub struct NetworkHandler{
    pub socket: UdpSocket,
}

impl NetworkHandler{
    pub fn new(socket: UdpSocket) -> NetworkHandler{
        NetworkHandler{
            socket,
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

