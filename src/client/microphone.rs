use std::{
    sync::mpsc::{
        Sender,
        Receiver,
    },
};
use cpal::{
    traits::{DeviceTrait, HostTrait},
    DeviceNameError,
    StreamConfig,
};

pub struct Microphone{
    pub microphone_recv_rx: Receiver<Vec<u8>>,
    pub microphone_trasmit_tx: Sender<Vec<u8>>,
    pub selected_microphone: cpal::Device,
    pub avaliable_microphones: std::iter::Filter<cpal::Devices, fn(&cpal::Device) -> bool>,
    pub supported_config: cpal::StreamConfig,
}

impl Microphone{
    pub fn new(microphone_recv_rx: Receiver<Vec<u8>>, microphone_trasmit_tx: Sender<Vec<u8>>) -> Microphone{
        let host: cpal::Host = cpal::default_host();
        let selected_microphone: cpal::Device = host.default_input_device().expect("No input device avaliable.");
        let avaliable_microphones: std::iter::Filter<cpal::Devices, fn(&cpal::Device) -> bool> = host.input_devices().unwrap();
        let mut supported_configs_range: cpal::SupportedOutputConfigs = selected_microphone.supported_output_configs()
        .expect("error while querying configs");
        let supported_config: cpal::StreamConfig = StreamConfig{buffer_size:cpal::BufferSize::Fixed(1500), sample_rate:cpal::SampleRate(44100), channels:2};
        Microphone{
            microphone_recv_rx,
            microphone_trasmit_tx,
            selected_microphone,
            avaliable_microphones,
            supported_config
        }
    }

    pub fn get_selected_microphone(&self) -> Result<std::string::String, DeviceNameError>{
        self.selected_microphone.name()
    }

    pub fn set_microphone(&mut self, microphone: cpal::Device){
        self.selected_microphone = microphone;
    }


}