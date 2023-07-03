use std::{
    sync::mpsc::{
        Sender,
        Receiver,
    },
};
use cpal::{
    traits::{DeviceTrait, HostTrait},
    DeviceNameError,
    StreamConfig
};

pub struct Speaker{
    pub speaker_recv_rx: Receiver<Vec<u8>>,
    pub speaker_trasmit_tx: Sender<Vec<u8>>,
    pub selected_speaker: cpal::Device,
    pub avaliable_speakers: std::iter::Filter<cpal::Devices, fn(&cpal::Device) -> bool>,
    pub supported_config: cpal::StreamConfig,
}

impl Speaker{
    pub fn new(speaker_recv_rx: Receiver<Vec<u8>>, speaker_trasmit_tx: Sender<Vec<u8>>) -> Speaker{
        let host: cpal::Host = cpal::default_host();
        let selected_speaker: cpal::Device = host.default_input_device().expect("No input device avaliable.");
        let avaliable_speakers: std::iter::Filter<cpal::Devices, fn(&cpal::Device) -> bool> = host.input_devices().unwrap();

        let mut supported_configs_range: cpal::SupportedOutputConfigs = selected_speaker.supported_output_configs()
        .expect("error while querying configs");
        let supported_config: cpal::StreamConfig = StreamConfig{buffer_size:cpal::BufferSize::Fixed(1500), sample_rate:cpal::SampleRate(44100), channels:2};
        Speaker{
            speaker_recv_rx,
            speaker_trasmit_tx,
            selected_speaker,
            avaliable_speakers,
            supported_config,
        }
    }

    pub fn get_selected_speaker(&self) -> Result<std::string::String, DeviceNameError>{
        self.selected_speaker.name()
    }

    pub fn set_speaker(&mut self, speaker: cpal::Device){
        self.selected_speaker = speaker;
    }

}