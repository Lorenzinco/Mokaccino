use std::sync::mpsc;
use std::thread;
use log::{debug, info, warn};
use serde_json::{Value, Result};
use std::fs;

#[path = "../utils/mod.rs"]
mod utils;
#[path = "./func/mod.rs"]
mod func;

pub struct Client{
    input: utils::terminal::user_interface::Input,
    output: utils::terminal::user_interface::Output,
    parser: func::parser::Parser,
    network_handler: utils::networking::network_handler::NetworkHandler,
    stream_handler: utils::networking::stream_handler::StreamHandler,
}

impl Client{
    pub fn new() -> Client{
        env_logger::init();
        debug!("Creating a new client");
        
        println!("Starting Mokaccino...");
        println!("Loading configuration file...");
        let config_file = fs::read_to_string("./config.json").expect("Unable to read config file");
        let (input_tx,output_rx) = mpsc::channel();
        let (parser_tx,parser_rx) = mpsc::channel();
        
        let config : Value = serde_json::from_str(&config_file).expect("Unable to parse config file");
        let port: u16 = config["port"].as_u64().unwrap() as u16;
        let username: String = config["username"].as_str().unwrap().to_string();
        //TODO
        let public_key: String = "".to_string();
        let private_key: String = "".to_string();
        Client{
            parser: func::parser::Parser::new(parser_tx,output_rx),
            input: utils::terminal::user_interface::Input::new(input_tx),
            output: utils::terminal::user_interface::Output::new(parser_rx),
            network_handler: utils::networking::network_handler::NetworkHandler::new(port,username,public_key,private_key),
            stream_handler: utils::networking::stream_handler::StreamHandler::new(port),
        }
    }


    pub fn run(mut self){
        let input: thread::JoinHandle<_> = thread::spawn(move || {
            loop{
                self.input.get_input();
            }
        });
        let output: thread::JoinHandle<_> = thread::spawn(move || {
            loop{
                self.output.print_output();
            }
        });
        let cmd: thread::JoinHandle<_> = thread::spawn(move || {
            loop{
                self.parser.get_input();
                self.parser.execute_command();
                self.parser.send_output();
            }
        });
    
        input.join().unwrap();
        output.join().unwrap();
        cmd.join().unwrap();
    }
}