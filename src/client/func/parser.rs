use std::sync::mpsc;
use log::debug;

pub struct Parser{
    input : String,
    output : String,
    tx: mpsc::Sender<String>,
    rx: mpsc::Receiver<String>,
}

impl Parser{
    //constructor
    pub fn new(tx: mpsc::Sender<String>, rx: mpsc::Receiver<String>) -> Parser{
        Parser{
            input : String::new(),
            output : String::new(),
            tx: tx,
            rx: rx,
        }
    }

    pub fn get_input(&mut self){
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
        self.input = self.input.trim().to_string();
        if self.input.is_empty(){
            self.output = String::from("");
        }
        else{
            let lines: Vec<&str> = self.input.split("\n").collect();
            let mut command = String::from(lines[0]);
            command = command.trim().to_string();
            debug!("command:{}",command);
            //execute command
            match command.as_str(){
                "help" => {
                    self.output = String::from(r"
                    help: shows this help
                    exit: exits the program
                    ");
                },
                "exit" => {
                    std::process::exit(0);
                },
                _ => {
                    self.output = String::from("command not found");
                }
            }
            //send output
            self.output = String::from("output");
        }
    }

    //setters
}