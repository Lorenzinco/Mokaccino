use log;
use std::io::ErrorKind;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use std::fs;
use std::thread;
use std::sync::{mpsc};
use std::time::Duration;
use crate::client::{commands, network::{serverConnection::ServerConnection, peerConnection::PeerConnection}};
use crate::utils::networking::packet::*;
use crate::utils::terminal::cli;

pub fn run() {
    cli::banner();

    log::debug!("Loading configuration file...");
    let config = fs::read_to_string("./config.json").expect("Unable to read config file");
    let config: serde_json::Value = serde_json::from_str(&config).expect("Unable to parse config file");
    let server: &str = config.get("server").unwrap().as_str().unwrap();
    let server: Vec<_> = server.to_socket_addrs().expect("Couldn't parse server address").collect();
    let server: SocketAddr = server[0];
    let username: String = config.get("username").unwrap().to_string();

    let socket = UdpSocket::bind("0.0.0.0:0").expect("Couldn't bind UDP socket");
    socket.set_nonblocking(true).expect("Couldn't set UDP socket in non-blocking mode");

    // IDEA: UI communicates with Server/Peer using a reserved opcode, which shall be discarded by the Network thread
    let (queue_tx, queue_rx) = mpsc::channel::<Packet>(); // outgoing packets go in queue_tx
    let (server_tx, server_rx) = mpsc::channel::<Packet>(); // Server receives data via server_rx
    let (peer_tx, peer_rx) = mpsc::channel::<Packet>(); // Peer receives data via peer_rx
    let (ui_tx, ui_rx) = mpsc::channel::<String>(); // UI receives data via ui_rx

    /*
        we create:
        - a network thread (will dispatch packets)
        - a server-connection thread (will react to packets coming from the server)
        - a peer-connection thread (will react to packets coming from the peer)
        - a CLI thread (will handle user input)
    */

    let mut conn_server = ServerConnection::new(server, server_rx, queue_tx.clone());
    let mut conn_peer: Option<PeerConnection> = None;
    conn_server.connect();

    // details about these threads shall still be decided
    let thread_server: thread::JoinHandle<_> = thread::spawn(move || loop {});
    let thread_peer: thread::JoinHandle<_> = thread::spawn(move || loop {});

    // sends and receives one packet at a time, shouldn't be slow... i guess...
    let thread_network: thread::JoinHandle<_> = thread::spawn(move || loop {
        // send one buffered packet
        let fetch = queue_rx.recv_timeout(Duration::from_millis(1)); // can't lose time here
        match fetch {
            Ok(packet) => {
                socket.send_to(&packet.payload, packet.addr).expect("Couldn't send UDP packet");
                log::debug!("Sending packet to {}", packet.addr);
            },
            _ => {} // no queued packets
        };

        // receive one buffered packet
        let mut buffer = EMPTY_PACKET;
        let result = socket.recv_from(&mut buffer);
        match result {
            Ok((number_of_bytes, source)) => {
                log::debug!("Received {} bytes from {}", number_of_bytes, source);
                let packet = Packet { addr: source, payload: buffer };
                if source == conn_server.addr {
                    server_tx.send(packet).expect("Internal thread communication error");
                } else {
                    peer_tx.send(packet).expect("Internal thread communication error");
                }
            },
            Err(ref err) if err.kind() != ErrorKind::WouldBlock => {
                cli::error(format!("Error while receiving UDP packet: {}", err).as_str());
                
            },
            _ => {} // no incoming packets
        };
    });

    // setup terminal thread
    let thread_cli: thread::JoinHandle<_> = thread::spawn(move || loop {
        let command: String = cli::input();
        let command: Vec<&str> = command.split_whitespace().collect();
        if command.len() < 1 {
            continue;
        }
        match command[0].to_ascii_lowercase().as_str() {
            "exit" => std::process::exit(0),
            "help" => commands::help(command[1..].to_vec()),
            "connect" => commands::connect(command[1..].to_vec(), &username, &mut conn_peer),
            "rawconnect" => cli::error("Not implemented"),
            "disconnect" => cli::error("Not implemented"),
            _ => cli::error("Unknown command. Type \"help\" for a list of allowed commands")
        }
    });
    
    thread_cli.join().unwrap();
    thread_network.join().unwrap();
}
