use std::{
    sync::mpsc::{
        Sender,
        Receiver,
    },
};

pub struct Voice{
    pub voice_recv_rx: Receiver<Vec<u8>>,
    pub voice_trasmit_tx: Sender<Vec<u8>>
}

impl Voice{
    pub fn new(voice_recv_rx: Receiver<Vec<u8>>, voice_trasmit_tx: Sender<Vec<u8>>) -> Voice{
        Voice{
            voice_recv_rx,
            voice_trasmit_tx,
        }
    }
}