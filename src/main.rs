mod utils;
mod client;
mod server;

use log::{debug, info, warn};
use std::env::args;


fn main() {    
    let mode = args().nth(0).expect("missing mode argument. Usage ./mokaccino [client|server]");
    match mode.as_str(){
        "client" => {
            info!("Starting client...");
            client::client::client();
        },
        "server" => {
            info!("Starting server...");
            //server::();
        },
        _ => {
            println!("Invalid mode. Usage mokaccino [-client|-server]");
            std::process::exit(1);
        }
    }
}

