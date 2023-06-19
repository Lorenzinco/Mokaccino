use log::debug;
use log::error;
use log::info;
use log::trace;
use log::warn;
use crate::client::utils::traits;

/*
Mokaccino's network handler class.
The class takes care of handling all the connections between all the peers.
The first connection always starts with the server, which is the first peer to be created.
The class also defines all the other functions to retrieve the data in order to connect to the other peers.
*/

enum IpVersion {
    IPV4,
    IPV6
}

struct IpAddr {
    ip_version: IpVersion,
    ip_address: String
}

struct Peer{
    active: bool,
    ip_address: IpAddr,
    port: u16,
    username: String,
    public_key: String,
}

//network handler class
pub struct NetworkHandler {
    peers: Vec<Peer>,
    server: Peer,
    port: u16,
    username: String,
    public_key: String,
    private_key: String,
}

impl NetworkHandler {
    //constructor
    pub fn new(port: u16, username: String, public_key: String, private_key: String) -> NetworkHandler {
        debug!("Creating a new network handler");
        NetworkHandler {
            peers: Vec::new(),
            server: Peer {
                active: true,
                port: 23232,
                username: String::from("server"),
                ip_address: IpAddr {
                    ip_version: IpVersion::IPV4,
                    ip_address: String::from("mokaccino.ddns.net"),
                },
                public_key: String::from("")
            },
            port: port,
            username: username,
            public_key: public_key,
            private_key: private_key
        }
    }

    //getters
    pub fn getPeers(&self) -> Vec<Peer> {
        debug!("Peer list requested");
        self.peers
    }

    pub fn getActivePeers(&self) -> Vec<Peer> {
        debug!("Active peer list requested");
        let mut active_peers: Vec<Peer> = Vec::new();
        for peer in self.peers {
            if peer.active {
                active_peers.push(peer);
            }
        }
        active_peers
    }
}

