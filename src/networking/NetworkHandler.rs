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
    ipv4,
    ipv6
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
        !debug!("Creating a new network handler");
        NetworkHandler {
            peers: Vec::new(),
            server: Peer {
                ip_address: IpAddr {
                    active: true,
                    ip_version: IpVersion::ipv4,
                    ip_address: String::from("mokaccino.ddns.net"),
                    port: 23232,
                    username: String::from("server"),
                }
            },
            port: port,
            username: username,
            public_key: public_key,
            private_key: private_key
        }
    }

    //getters
    pub fn get_peers(&self) -> Vec<Peer> {
        !debug!("Current peers");
        self.peers
    }

}

