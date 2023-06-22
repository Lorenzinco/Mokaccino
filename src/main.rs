mod utils;
mod client;
mod server;

use crate::client::client::Client;

use log::{info, error};
use std::env::args;

fn main() {
    env_logger::init();
    
    ctrlc::set_handler(move || {
        print!("\r\x1b[2K");
        println!("\x1b[95mExiting...\x1b[0m");
        println!("Bye!");
        std::process::exit(0);
    })
    .expect("Couldn't bind handler to CTRL-C");

    match args().nth(1) {
        None => {
            error!("Usage: ./mokaccino <client/server>");
            std::process::exit(1);
        },
        Some(mode) => {
            match mode.to_lowercase().as_str() {
                "client" => {
                    info!("Starting Mokaccino in client mode...");
                    let client: Client = Client::new();
                    client.run();
                }
                "server" => {
                    info!("Starting Mokaccino in server mode...");
                    // server::();
                }
                _ => {
                    error!("Usage: ./mokaccino <client/server>");
                    std::process::exit(1);
                }
            }
        }
    }
}
