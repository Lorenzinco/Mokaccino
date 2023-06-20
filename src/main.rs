mod utils;
mod client;
mod server;

use crate::client::client::Client;

use log::{debug, info, warn};
use std::env::args;
use std::thread;


fn main() {
    //signal handler
    ctrlc::set_handler(move || {
        print!("\r\x1b[2K");
        println!("\x1b[95mExiting...\x1b[0m");
        println!("Bye!");
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");

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

