use mokaccino::client::client::{Client};

use std::env::args;

fn main() {
    env_logger::init();
    
    ctrlc::set_handler(move || {
        cli::info("\r\x1b[2K\x1b[95mExiting...\x1b[0m");
        println!("Bye!");
        std::process::exit(0);
    })
    .expect("Couldn't bind handler to CTRL-C");

    match args().nth(1) {
        None => {
            cli::error("Usage: ./mokaccino <client/server>");
            std::process::exit(1);
        },
        Some(mode) => {
            match mode.to_lowercase().as_str() {
                "client" => {
                    cli::info("Starting Mokaccino in client mode...");
                    let client = Client::new();
                    client::start();
                },
                "server" => {
                    cli::info("Starting Mokaccino in server mode...");
                    // server::run();
                },
                _ => {
                    cli::error("Usage: ./mokaccino <client/server>");
                    std::process::exit(1);
                }
            }
        }
    }
}
