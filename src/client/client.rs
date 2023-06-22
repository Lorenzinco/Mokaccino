use log;
use std::net::{SocketAddr,ToSocketAddrs};
use std::thread;
use std::fs;
use std::sync::mpsc;
use utils::networking::socket;
use utils::terminal::cli;

use crate::utils;
use crate::client::network::{self, server};
use crate::client::commands;

pub struct Client {
    ready: bool,
    socket: socket::Socket,
    server: network::server::Server,
    peer: Option<network::peer::Peer>
}

impl Client {
    pub fn new() -> Client {
        log::debug!("Loading configuration file...");
        let config = fs::read_to_string("./config.json").expect("Unable to read config file");
        let config: serde_json::Value = serde_json::from_str(&config).expect("Unable to parse config file");
        
        let port: u16 = u16::try_from(config.get("port").unwrap().as_u64().unwrap()).expect("Invalid client port");
        let username: &str = config.get("username").unwrap().as_str().unwrap();
        let server_details: &str = config.get("server").unwrap().as_str().unwrap();
        let server: Vec<_> = server_details.to_socket_addrs().expect("Couldn't parse server address").collect();
        let server: SocketAddr = server[0];
        
        let (socket_tx, socket_rx) = mpsc::channel(); // outgoing packets in socket_tx
        let (server_tx, server_rx) = mpsc::channel(); // Server receives data via server_rx
        let (peer_tx, peer_rx) = mpsc::channel(); // Peer receives data via peer_rx
        Client {
            ready: false,
            socket: socket::Socket::new(port, server, socket_rx, server_tx, peer_tx),
            server: network::server::Server::connect(server, server_rx, socket_tx),
            peer: None,
            
        }
    }

    pub fn run(mut self) {
        cli::banner();
        let net: thread::JoinHandle<_> = thread::spawn(move || loop {
            // network loop
        });
        let cli: thread::JoinHandle<_> = thread::spawn(move || loop {
            let command_line = cli::input().to_ascii_lowercase();
            let command: Vec<&str> = command_line.split_whitespace().collect();
            if command.len() < 1 {
                continue;
            }
            match command[0] {
                "help" => cli::output(commands::help(command[1..].to_vec())),
                //"continue" => cli::output(commands::connect(command[1..].to_vec(),&self.peer)),
                _ => cli::output("Unknown command, type help for a list of commands")

            }
            
        });
        cli.join().unwrap();
    }
}
