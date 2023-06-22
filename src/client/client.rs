use log::debug;
use std::thread;
use std::fs;
use std::sync::mpsc;
use std::net::ToSocketAddrs;
use utils::networking;
use utils::terminal::cli;

#[path = "../utils/mod.rs"]
mod utils;
#[path = "./network/mod.rs"]
mod network;

pub struct Client {
    ready: bool,
    socket: networking::socket::Socket,
    server: network::server::Server,
    peer: Option<network::peer::Peer>
}

impl Client {
    pub fn new() -> Client {
        debug!("Loading configuration file...");
        let config_file = fs::read_to_string("./config.json").expect("Unable to read config file");
        let config: serde_json::Value = serde_json::from_str(&config_file).expect("Unable to parse config file");
        
        let cfg_username: String = config.get("username").unwrap().as_str().unwrap();
        let cfg_local_port: String = config.get("local_port").unwrap(); // TODO: va convertito in u16
        let cfg_server_host: String = config.get("server_host").unwrap().as_str().unwrap();
        let cfg_server_port: String = config.get("server_port").unwrap(); // TODO: va convertito in u16

        let server_addr = match cfg_server_host.to_socket_addrs().unwrap().next() {
            None => {
                error!("Fatal error! Couldn't resolve server hostname {}", host);
                std::process::exit(1);
            },
            Some(addr) => addr
        };
        
        let (socket_tx, socket_rx) = mpsc::Channel(); // outgoing packets in socket_tx
        let (server_tx, server_rx) = mpsc::channel(); // Server receives data via server_rx
        let (peer_tx, peer_rx) = mpsc::channel(); // Peer receives data via peer_rx
        // CODICE NON FINITO! SONO LE 3 DI NOTTE
        Client {
            ready: false,
            socket: networking::socket::Socket::new(cfg_local_port, server_addr, socket_rx, server_tx, peer_tx),
            server: network::server::connect(server_addr),
            peer: None
        }
    }

    pub fn run(mut self) {
        cli::banner();
        let net: thread::JoinHandle<_> = thread::spawn(move || loop {
            // network loop
        });
        let cli: thread::JoinHandle<_> = thread::spawn(move || loop {
            if(self.ready) {
                let command: String = cli::input();
                match command {
                    _ => println!("Not implemented")
                }
            }
        });
        cli.join().unwrap();
    }
}
