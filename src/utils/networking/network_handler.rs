use std::net::IpAddr;
use std::collections::HashMap;
use log::error;
use log::info;
use log::debug;
/*
Mokaccino's network handler class.
The class takes care of handling all the connections between all the peers.
The first connection always starts with the server, which is the first peer to be created.
The class also defines all the other functions to retrieve the data in order to connect to the other peers.
*/

#[derive(Clone)]
pub struct Peer{
    timeout: u64,
    ip_address: IpAddr,
    port: u16,
    username: String,
    public_key: String,
}

//network handler class
#[derive(Clone)]
pub struct NetworkHandler {
    peers: HashMap<String,Peer>,
    pending_peers: HashMap<String,Peer>,
    username: String,
    public_key: String,
    private_key: String,
}



impl NetworkHandler {
    //constructor
    pub fn new(port: u16, username: String, public_key: String, private_key: String) -> NetworkHandler {
        debug!("Creating a new network handler");
        NetworkHandler {
            peers: HashMap::new(),
            pending_peers: HashMap::new(),
            username: username,
            public_key: public_key,
            private_key: private_key
        }
    }

    //getters
    pub fn get_peers(&self) -> Vec<Peer> {
        debug!("Peer list requested:");
        let mut peer_list: Vec<Peer> = Vec::new();
        for peer in self.peers.clone() {
            debug!("Peer: {}", peer.1.username);
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
    
    pub fn get_public_key(&self) -> String {
        debug!("Public key requested");
        self.public_key.clone()
    }

    pub fn get_private_key(&self) -> String {
        debug!("Private key requested");
        self.private_key.clone()
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

    pub fn set_public_key(&mut self, public_key: String) {
        info!("Public key set to {}", public_key);
        self.public_key = public_key;
    }

    pub fn set_private_key(&mut self, private_key: String) {
        info!("Private key set to {}", private_key);
        self.private_key = private_key;
    }

    //functions
    pub fn add_peer(&mut self, peer: Peer) {
        let key = peer.username.clone();
        info!("Peer {} added", key);
        self.peers.insert(key,peer).expect("Error adding peer");
    }

    pub fn remove_peer(&mut self, peer: Peer) {
        self.peers.remove(&peer.username).expect("Error removing peer");
        info!("Peer {} removed", peer.username);
    }
}