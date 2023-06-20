use std::sync::mpsc;
use log::debug;

pub struct parser{
    input : String,
    output : String,
    tx: mpsc::Sender<String>,
    rx: mpsc::Receiver<String>,
}

impl parser{
    //constructor
    pub fn new(tx: mpsc::Sender<String>, rx: mpsc::Receiver<String>) -> parser{
        parser{
            input : String::new(),
            output : String::new(),
            tx: tx,
            rx: rx,
        }
    }

    pub fn parse_input(&mut self){
            //get input from terminal
            self.input = self.rx.recv().unwrap();
            debug!("parser received:{}",self.input);
    }

    pub fn send_output(&mut self){
        self.tx.send(self.output.clone()).unwrap();
        debug!("{}",self.output)
    }

    pub fn execute_command(&mut self){
        //parse input
        //execute command
        //send output
    }

    //setters
}