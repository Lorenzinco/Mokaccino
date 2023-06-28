use std::{
    sync::mpsc::{Sender, Receiver},
};

pub struct Client{
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
    
}

impl Client{
    pub fn start(){
        
    }

    pub fn new() -> Client{
        Client{}
    }
}