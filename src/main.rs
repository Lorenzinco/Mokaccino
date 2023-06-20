use log::{debug, info, warn};
mod utils;
use std::sync::mpsc;
use std::thread;

fn main() {
    env_logger::init();
    info!("Starting Mokaccino...");
    let (input_tx,output_rx) = mpsc::channel();
    let (parser_tx,parser_rx) = mpsc::channel();
    
    let mut parser: utils::terminal::parser::Parser = utils::terminal::parser::Parser::new(parser_tx,output_rx);
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

