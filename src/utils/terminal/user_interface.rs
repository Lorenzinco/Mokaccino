use const_format::formatcp;
use std::io::{self, Write};
use std::env;
use std::sync::mpsc;
use log::debug;
/*
    Definition of the user interface class, which reads the user input in the terminal
    as a separate thread, and sends the input to the main thread through a channel.
    The main thread then processes the input and sends the output to the user interface.
*/

pub const BANNER:&'static str = formatcp!("
\x1b[92m __    __ \x1b[0m    ______     __  __     ______     ______     ______     __     __   __     ______    
\x1b[92m/\\  -./  \\\x1b[0m   /\\  __ \\   /\\ \\/ /    /\\  __ \\   /\\  ___\\   /\\  ___\\   /\\ \\   /\\  -.\\ \\   /\\  __ \\   
\x1b[92m\\ \\ \\-./\\ \\\x1b[0m  \\ \\ \\/\\ \\  \\ \\  _ -.  \\ \\  __ \\  \\ \\ \\____  \\ \\ \\____  \\ \\ \\  \\ \\ \\-.  \\  \\ \\ \\/\\ \\  
\x1b[92m \\ \\_\\ \\ \\_\\\x1b[0m  \\ \\_____\\  \\ \\_\\ \\_\\  \\ \\_\\ \\_\\  \\ \\_____\\  \\ \\_____\\  \\ \\_\\  \\ \\_\\\\ \\_\\  \\ \\_____\\ 
\x1b[92m  \\/_/  \\/_/ \x1b[0m  \\/_____/   \\/_/\\/_/   \\/_/\\/_/   \\/_____/   \\/_____/   \\/_/   \\/_/ \\/_/   \\/_____/ 
   \x1b[94mBuild version: {} - Authors:{} - Copyright © \x1b[92mTRX\x1b[94m, all rights reserved. \n\x1b[0mType help for a list of commands\n",env!("CARGO_PKG_VERSION"),env!("CARGO_PKG_AUTHORS"));

pub const HELP:&str = r"

";

pub struct Input{
    input : String,
    tx: mpsc::Sender<String>,
}

pub struct Output{
    output : String,
    rx: mpsc::Receiver<String>,
}

impl Input{
    //constructor
    pub fn new(tx: mpsc::Sender<String>) -> Input{
        Input{
            input : String::new(),
            tx: tx
        }
    }

    pub fn get_input(&mut self){
        self.input.clear();
        print!("\x1b[92m>\x1b[0m ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut self.input).unwrap();
        debug!("input: {}",self.input.clone());
        if self.input!=String::from(""){//input isnt empty
            self.tx.send(self.input.clone()).unwrap();
        }
    }

}

impl Output{
    //constructor
    pub fn new(rx: mpsc::Receiver<String>) -> Output{
        println!("{}",BANNER);
        Output{
            output : String::new(),
            rx: rx
        }
    }

    pub fn print_output(&mut self){
        self.output = self.rx.recv().unwrap();
        print!("{}",self.output.clone());
    }

}