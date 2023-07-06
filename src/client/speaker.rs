use cpal::{
    traits::{DeviceTrait, HostTrait},
    DeviceNameError,
};

pub struct Speaker{
    pub selected_speaker: cpal::Device,
    pub avaliable_speakers: std::iter::Filter<cpal::Devices, fn(&cpal::Device) -> bool>,
    pub config: cpal::StreamConfig,
}

impl Speaker{
    pub fn new() -> Speaker{
        let host: cpal::Host = cpal::default_host();
        let selected_speaker: cpal::Device = host.default_output_device().expect("No output device avaliable.");
        let avaliable_speakers: std::iter::Filter<cpal::Devices, fn(&cpal::Device) -> bool> = host.output_devices().unwrap();

        let mut supported_configs_range: cpal::SupportedOutputConfigs = selected_speaker.supported_output_configs()
            .expect("error while querying configs");
        //get the supported config that has 1 channel and the highest sample rate
        let supported_config = supported_configs_range.next()
            .expect("no supported config?!")
            .with_max_sample_rate();
        let mut config: cpal::StreamConfig = supported_config.config();
        config.channels = 1;
        Speaker{
            selected_speaker,
            avaliable_speakers,
            config,
        }
    }

    pub fn get_selected_speaker(&self) -> Result<std::string::String, DeviceNameError>{
        self.selected_speaker.name()
    }

    pub fn set_speaker(&mut self, speaker: cpal::Device){
        self.selected_speaker = speaker;
    }

}