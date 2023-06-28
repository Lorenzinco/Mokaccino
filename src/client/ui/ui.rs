use std::{
    sync::mpsc::{
        Sender,
        Receiver,
    },
};

pub struct Ui{
    pub ui_input_tx: Sender<String>,
    pub ui_display_rx: Receiver<String>,
}

impl Ui{
    pub fn new(ui_input_tx: Sender<String>, ui_display_rx: Receiver<String>) -> Ui{
        Ui{
            ui_input_tx,
            ui_display_rx,
        }
    }
}