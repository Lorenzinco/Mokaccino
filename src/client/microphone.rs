use std::{
    sync::mpsc::{
        Sender,
        Receiver,
    },
};
use soundio::{
    Context::new,
};

pub struct Microphone{
    pub microphone_recv_rx: Receiver<Vec<u8>>,
    pub microphone_trasmit_tx: Sender<Vec<u8>>,
    pub selected_mic: soundio::Device,
    pub avaliable_microphones: Vec<soundio::Device>,
}

impl Microphone{
    pub fn new(microphone_recv_rx: Receiver<Vec<u8>>, microphone_trasmit_tx: Sender<Vec<u8>>) -> Microphone{
        let mut context = new();
        avaliable_microphones = context.input_devices();
        selected_mic = context.default_input_device();
        microphone{
            microphone_recv_rx,
            microphone_trasmit_tx,
            selected_mic,
            avaliable_microphones,
        }
    }

    pub fn get_selected_mic(&self) -> soundio::Device{
        self.selected_mic
    }

    pub fn set_select_mic(&mut self, mic: soundio::Device){
        self.selected_mic = mic
    }

    pub fn get_avaliable_mics(&self) -> Vec<soundio::Device>{
        self.avaliable_microphones
    }
}