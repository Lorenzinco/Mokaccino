use log;
use std::net::{SocketAddr, ToSocketAddrs};
use std::thread;
use std::fs;
use std::sync::mpsc::{self, Sender, Receiver};
use crate::client::{commands, network::{server::Server, peer::Peer}};
use crate::utils::networking::packet::Packet;
use crate::utils::networking::socket::Socket;
use crate::utils::terminal::cli;

pub struct Client {
    socket: Socket,
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
        Client {
            online: false,
            username: username,
            socket: Socket::new(server),
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
        let net: thread::JoinHandle<_> = thread::spawn(move || loop {
            // non capisco l'errore che esce qui, nè come risolverlo
            self.socket.send(self.socket_rx);
            self.socket.receive(self.server_tx, self.peer_tx);
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
        net.join().unwrap();
    }
}
