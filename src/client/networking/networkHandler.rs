use log::debug;
use log::error;
use log::info;
use log::trace;
use log::warn;

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
    ipVersion: IpVersion,
    ipAddress: String
}

struct Peer{
    active: bool,
    ipAddress: IpAddr,
    port: u16,
    username: String,
    publicKey: String,
}

//network handler class
pub struct NetworkHandler {
    peers: Vec<Peer>,
    server: Peer,
    port: u16,
    username: String,
    publicKey: String,
    privateKey: String,
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
                ipAddress: IpAddr {
                    ipVersion: IpVersion::IPV4,
                    ipAddress: String::from("mokaccino.ddns.net"),
                },
                publicKey: String::from("")
            },
            port: port,
            username: username,
            publicKey: public_key,
            privateKey: private_key
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
        for peer in self.peers.into_iter() {
            if peer.active {
                active_peers.push(peer);
            }
        }
        active_peers
    }
}

