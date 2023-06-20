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
 __    __     ______     __  __     ______     ______     ______     __     __   __     ______    
/\\  -./  \\   /\\  __ \\   /\\ \\/ /    /\\  __ \\   /\\  ___\\   /\\  ___\\   /\\ \\   /\\  -.\\ \\   /\\  __ \\   
\\ \\ \\-./\\ \\  \\ \\ \\/\\ \\  \\ \\  _ -.  \\ \\  __ \\  \\ \\ \\____  \\ \\ \\____  \\ \\ \\  \\ \\ \\-.  \\  \\ \\ \\/\\ \\  
 \\ \\_\\ \\ \\_\\  \\ \\_____\\  \\ \\_\\ \\_\\  \\ \\_\\ \\_\\  \\ \\_____\\  \\ \\_____\\  \\ \\_\\  \\ \\_\\\\ \\_\\  \\ \\_____\\ 
  \\/_/  \\/_/   \\/_____/   \\/_/\\/_/   \\/_/\\/_/   \\/_____/   \\/_____/   \\/_/   \\/_/ \\/_/   \\/_____/ 
   Build version: {} - Authors:{} - Copyright © TRX, all rights reserved. \nType help for a list of commands\n",env!("CARGO_PKG_VERSION"),env!("CARGO_PKG_AUTHORS"));

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
        print!("> ");
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