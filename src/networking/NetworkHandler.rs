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
    ip_address: IpAddr,
    port: u16,
    username: String,
    private_key: String,
}