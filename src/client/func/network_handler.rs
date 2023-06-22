use log::debug;
use log::error;
use log::info;
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::mpsc;
/*
Mokaccino's network handler class.
The class takes care of handling all the connections between all the peers.
The first connection always starts with the server, which is the first peer to be created.
The class also defines all the other functions to retrieve the data in order to connect to the other peers.
*/

#[derive(Clone)]
pub struct Peer {
    timeout: u64,
    ip_address: IpAddr,
    port: u16,
    username: String,
    public_key: String,
}

//network handler class
pub struct NetworkHandler {
    peers: HashMap<String, Peer>,
    pending_peers: HashMap<String, Peer>,
    username: String,
    public_key: String,
    private_key: String,
    stream_handler: StreamHandler,
    tx: mpsc::Sender<String>,
    rx: mpsc::Receiver<String>,
}

impl NetworkHandler {
    //constructor
    pub fn new(
        username: String,
        public_key: String,
        private_key: String,
        stream_tx: mpsc::Sender<String>,
        stream_rx: mpsc::Receiver<String>,
        network_tx: mpsc::Sender<String>,
        network_rx: mpsc::Receiver<String>,
    ) -> NetworkHandler {
        debug!("Creating a new network handler");
        NetworkHandler {
            peers: HashMap::new(),
            pending_peers: HashMap::new(),
            username: username,
            public_key: public_key,
            private_key: private_key,
            stream_handler: StreamHandler::new(stream_rx, network_tx),
            tx: stream_tx,
            rx: network_rx,
        }
    }

    //getters
    pub fn get_peers(&self) -> Vec<Peer> {
        debug!("Peer list requested:");
        let mut peer_list: Vec<Peer> = Vec::new();
        for peer in self.peers.clone() {
            peer_list.push(peer.1);
        }
        peer_list
    }

    pub fn get_active_peers(&self) -> Vec<Peer> {
        debug!("Active peer list requested:");
        let mut active_peer_list: Vec<Peer> = Vec::new();
        for peer in self.peers.clone() {
            debug!("Peer: {}", peer.1.username);
            active_peer_list.push(peer.1);
        }
        active_peer_list
    }

    pub fn get_username(&self) -> String {
        debug!("Username requested");
        self.username.clone()
    }

    //setters
    pub fn set_username(&mut self, username: String) {
        info!("Username set to {}", username);
        self.username = username;
    }

    //functions
    pub fn add_peer(&mut self, peer: Peer) {
        let key = peer.username.clone();
        info!("Peer {} added", key);
        self.peers.insert(key, peer).expect("Error adding peer");
    }

    pub fn remove_peer(&mut self, peer: Peer) {
        self.peers
            .remove(&peer.username)
            .expect("Error removing peer");
        info!("Peer {} removed", peer.username);
    }

    pub fn connect(&mut self, username: String) -> String {
        let mut output = String::new();
        let dest: &Peer = self.peers.get(&username).expect("Error getting peer");
        self.stream_handler.send(
            format!(
                "{} {}",
                get_op_code(Command::Connect(username)),
                self.username
            ),
            dest.ip_address,
            dest.port,
        );
        output.push_str(&format!(
            "Sent: {} {}",
            get_op_code(Command::Connect(String::new())),
            self.username
        ));
        output
    }
}
