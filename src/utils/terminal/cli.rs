use log;
use std::env;
use std::io::{self, Write};
use const_format::formatcp;

pub const BANNER: &'static str = formatcp!("
\x1b[92m __    __ \x1b[0m    ______     __  __     ______     ______     ______     __     __   __     ______    
\x1b[92m/\\  -./  \\\x1b[0m   /\\  __ \\   /\\ \\/ /    /\\  __ \\   /\\  ___\\   /\\  ___\\   /\\ \\   /\\  -.\\ \\   /\\  __ \\   
\x1b[92m\\ \\ \\-./\\ \\\x1b[0m  \\ \\ \\/\\ \\  \\ \\  _ -.  \\ \\  __ \\  \\ \\ \\____  \\ \\ \\____  \\ \\ \\  \\ \\ \\-.  \\  \\ \\ \\/\\ \\  
\x1b[92m \\ \\_\\ \\ \\_\\\x1b[0m  \\ \\_____\\  \\ \\_\\ \\_\\  \\ \\_\\ \\_\\  \\ \\_____\\  \\ \\_____\\  \\ \\_\\  \\ \\_\\\\ \\_\\  \\ \\_____\\ 
\x1b[92m  \\/_/  \\/_/ \x1b[0m  \\/_____/   \\/_/\\/_/   \\/_/\\/_/   \\/_____/   \\/_____/   \\/_/   \\/_/ \\/_/   \\/_____/ 
   \x1b[94mVersion: {} - Authors: {} - Copyright © \x1b[92mTRX\x1b[94m, all rights reserved. \n\x1b[0mType \"help\" for a list of commands.\n", env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS"));

pub fn banner() {
    println!("{}", BANNER);
}

pub fn input() -> String {
    print!("\r\x1b[2K\x1b[92m>\x1b[0m ");
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer
}

pub fn output(s: &str) {
    println!("{}", s);
    io::stdout().flush().unwrap();
}

pub fn info(s: &str) {
    log::info!("\r\x1b[2K[\x1b[94m*\x1b[0m] {}", s);
}

pub fn error(s: &str) {
    log::error!("\r\x1b[2K[\x1b[91m*\x1b[0m] {}", s);
}