use cpal::{
    traits::{DeviceTrait, HostTrait},
    DeviceNameError,
};

pub struct Microphone{
    pub selected_microphone: cpal::Device,
    pub avaliable_microphones: std::iter::Filter<cpal::Devices, fn(&cpal::Device) -> bool>,
    pub config: cpal::StreamConfig,
}

impl Microphone{
    pub fn new() -> Microphone{
        let host: cpal::Host = cpal::default_host();
        let selected_microphone: cpal::Device = host.default_input_device().expect("No input device avaliable.");
        let avaliable_microphones: std::iter::Filter<cpal::Devices, fn(&cpal::Device) -> bool> = host.input_devices().unwrap();
        let mut supported_configs_range: cpal::SupportedInputConfigs = selected_microphone.supported_input_configs()
            .expect("error while querying configs");
        let supported_configs_range_debug: cpal::SupportedInputConfigs = selected_microphone.supported_input_configs()
            .expect("error while querying configs");
        //print supported configs
        log::debug!("Supported configs:");
        for supported_config in supported_configs_range_debug{
            log::debug!("{:?}", supported_config);
        }
        let supported_config = supported_configs_range.next()
            .expect("no supported config?!")
            .with_max_sample_rate();
        let mut config: cpal::StreamConfig = supported_config.config();
        log::debug!("Microphone config: {:?}", config);
        config.channels = 1;
        config.buffer_size = cpal::BufferSize::Fixed(16);
        Microphone{
            selected_microphone,
            avaliable_microphones,
            config
        }
    }

    pub fn get_selected_microphone(&self) -> Result<std::string::String, DeviceNameError>{
        self.selected_microphone.name()
    }

    pub fn set_microphone(&mut self, microphone: cpal::Device){
        self.selected_microphone = microphone;
    }


}