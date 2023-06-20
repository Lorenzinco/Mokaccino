mod utils;
mod client;
mod server;

use crate::client::client::Client;

use log::{debug, info, warn};
use std::env::args;


fn main() {    
    let mode = args().nth(1).expect("missing mode argument. Usage ./mokaccino [client|server]");
    match mode.as_str(){
        "client" => {
            info!("Starting client...");
            let mut client: Client = Client::new();
            client.run();
        },
        "server" => {
            info!("Starting server...");
            //server::();
        },
        _ => {
            println!("Invalid mode. Usage mokaccino [client|server]");
            std::process::exit(1);
        }
    }
}

