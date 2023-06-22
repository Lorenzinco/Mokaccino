use log;
use std::io::ErrorKind;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use std::thread;
use std::fs;
use std::sync::Arc;
use std::sync::mpsc::{self, Sender, Receiver};
use crate::client::{commands, network::{server::Server, peer::Peer}};
use crate::utils::networking::packet::Packet;
use crate::utils::networking::protocol::*;
use crate::utils::terminal::cli;

pub struct Client {
    // socket: Socket,
    socket: Arc<UdpSocket>,
    server: Server,
    socket_tx: Sender<Packet>,
    socket_rx: Receiver<Packet>,
    server_tx: Sender<Packet>,
    peer_tx: Sender<Packet>,
    peer_rx: Receiver<Packet>, // Peer struct will take ownership of this
    pub peer: Option<Peer>,
    pub online: bool,
    pub username: String
}

impl Client {
    pub fn new() -> Client {
        log::debug!("Loading configuration file...");
        let config = fs::read_to_string("./config.json").expect("Unable to read config file");
        let config: serde_json::Value = serde_json::from_str(&config).expect("Unable to parse config file");
        
        let username: String = config.get("username").unwrap().to_string();
        let server: &str = config.get("server").unwrap().as_str().unwrap();
        let server: Vec<_> = server.to_socket_addrs().expect("Couldn't parse server address").collect();
        let server: SocketAddr = server[0];

        let (socket_tx, socket_rx) = mpsc::channel(); // outgoing packets in socket_tx
        let (server_tx, server_rx) = mpsc::channel(); // Server receives data via server_rx
        let (peer_tx, peer_rx) = mpsc::channel(); // Peer receives data via peer_rx

        let socket = UdpSocket::bind("0.0.0.0:0").expect("Couldn't bind UDP socket");
        socket.set_nonblocking(true).expect("Couldn't set UDP socket in non-blocking mode");

        Client {
            online: false,
            username: username,
            socket: Arc::new(socket),
            server: Server::connect(server, server_rx, socket_tx.clone()),
            peer: None,
            socket_tx: socket_tx,
            socket_rx: socket_rx,
            server_tx: server_tx,
            peer_tx: peer_tx,
            peer_rx: peer_rx
        }
    }

    pub fn run(mut self) {
        cli::banner();
        let socket_recv = self.socket.clone();
        let socket_send = self.socket.clone();
        let net_recv: thread::JoinHandle<_> = thread::spawn(move || loop {
            let mut buffer: [u8; PACKET_MAX_LENGTH] = [0; PACKET_MAX_LENGTH];
            let result = socket_recv.recv_from(&mut buffer);
            match result {
                Ok((number_of_bytes, source)) => {
                    log::debug!("Received {} bytes from {}", number_of_bytes, source);
                    let packet = Packet {
                        addr: source,
                        payload: buffer
                    };
                    if source == self.server.addr {
                        self.server_tx.send(packet).expect("Couldn't queue UDP packet while communicating with server");
                    } else {
                        self.peer_tx.send(packet).expect("Couldn't queue UDP packet while communicating with peer");
                    }
                },
                Err(ref err) if err.kind() != ErrorKind::WouldBlock => {
                    cli::error(format!("Error while receiving from UDP socket: {}", err).as_str());
                    
                },
                _ => {} // no incoming packets
        };
        });
        let net_send: thread::JoinHandle<_> = thread::spawn(move || loop {
            let packet = self.socket_rx.recv().unwrap();
            log::debug!("Sending packet to {}", packet.addr);
            socket_send.send_to(&packet.payload, packet.addr).expect("Couldn't send UDP packet");
        });
        let cli: thread::JoinHandle<_> = thread::spawn(move || loop {
            if self.online {
                let command: String = cli::input();
                let command: Vec<&str> = command.split_whitespace().collect();
                if command.len() < 1 {
                    continue;
                }
                match command[0].to_ascii_lowercase().as_str() {
                    "exit" => std::process::exit(0),
                    "help" => commands::help(command[1..].to_vec()),
                    other => {
                        // this is needed to catch if client went offline while waiting for input
                        if self.online {
                            match other {    
                                "connect" => commands::connect(command[1..].to_vec(), &self.username, &mut self.peer),
                                _ => cli::error("Unknown command. Type \"help\" for a list of allowed commands")
                            }
                        } else {
                            cli::error("Please wait until connection is restored before attempting to execute commands.");
                        }
                        
                    }
                }
            }
        });
        cli.join().unwrap();
        net_recv.join().unwrap();
        net_send.join().unwrap();
    }
}
