use log::debug;
use std::net::{IpAddr, UdpSocket};

pub struct StreamHandler {
    stream: UdpSocket,
}

impl StreamHandler {
    //constructor
    pub fn new(port: u16) -> StreamHandler {
        debug!("Creating a new stream handler");
        StreamHandler {
            stream: UdpSocket::bind(format!("127.0.0.1:{}", port)).expect("Error binding socket")
        }
    }

    //getters
    pub fn get_port(&self) -> u16 {
        debug!("Port requested");
        self.stream.local_addr().unwrap().port()
    }

    //functions
    pub fn send(&self, payload: String, ip_address: IpAddr, port: u16) {
        debug!("Sending message to {}:{}", ip_address, port);
        self.stream.send_to(payload.as_bytes(), format!("{}:{}", ip_address, port)).expect("Error sending message");
    }

    pub fn receive(&self) -> String {
        debug!("Message incoming");
        let mut buffer = [0; 1024];
        let (number_of_bytes, src_addr) = self.stream.recv_from(&mut buffer).expect("Error receiving message");
        debug!("Received {} bytes from {}", number_of_bytes, src_addr);
        String::from_utf8_lossy(&buffer[..number_of_bytes]).to_string()
    }
}

