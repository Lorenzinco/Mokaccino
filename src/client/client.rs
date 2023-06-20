use std::sync::mpsc;
use std::thread;
use log::{debug, info, warn};

#[path = "../utils/mod.rs"]
mod utils;
#[path = "./func/mod.rs"]
mod func;


pub fn client(){
    env_logger::init();
    info!("Starting Mokaccino...");
    let (input_tx,output_rx) = mpsc::channel();
    let (parser_tx,parser_rx) = mpsc::channel();
    
    let mut parser: func::parser::Parser = func::parser::Parser::new(parser_tx,output_rx);
    let mut input: utils::terminal::user_interface::Input = utils::terminal::user_interface::Input::new(input_tx);
    let mut output: utils::terminal::user_interface::Output = utils::terminal::user_interface::Output::new(parser_rx);

    let input: thread::JoinHandle<_> = thread::spawn(move || {
        loop{
            input.get_input();
        }
    });
    let output: thread::JoinHandle<_> = thread::spawn(move || {
        loop{
            output.print_output();
        }
    });
    let cmd: thread::JoinHandle<_> = thread::spawn(move || {
        loop{
            parser.get_input();
            parser.execute_command();
            parser.send_output();
        }
    });

    input.join().unwrap();
    output.join().unwrap();
    cmd.join().unwrap();
}