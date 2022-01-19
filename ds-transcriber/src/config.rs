//! Initialises recording device and audio stream's silence level

use cpal::{
    traits::{DeviceTrait, HostTrait},
    Device, SampleRate, SupportedStreamConfig,
};

///
/// # Input device configuration
/// Gets data ready to begin recording

pub struct StreamConfig {
    device: Device,
    config: SupportedStreamConfig,
    silence_level: i32,
}

#[cfg(test)]
mod tests {
    use crate::config::get_config;
    use cpal::traits::HostTrait;

    #[test]
    fn supported_device() {
        let host = cpal::default_host();
        let device = host.default_input_device().expect("no input device found");

        assert_eq!(1, get_config(&device).channels());
    }
}

impl StreamConfig {
    /// Creates a new stream configuration:
    pub fn new(silence_level: i32) -> Self {
        println!("In stream config preparing to get default device");
        let device = get_default_device();
        println!("Got default device");
        let config = get_config(&device);
        StreamConfig {
            config,
            device,
            silence_level,
        }
    }

    /// Returns the device in use:
    pub fn device(&self) -> &Device {
        &self.device
    }

    /// Returns SupportedStreamConfig (.config() returns the configuration we use on our stream)
    pub fn supported_config(&self) -> &SupportedStreamConfig {
        &self.config
    }

    /// Returns the silence level
    pub fn silence_level(&self) -> i32 {
        self.silence_level
    }
}

///Returns the configuration of the mono channel
fn get_config(device: &Device) -> SupportedStreamConfig {
    println!("In get_config");
    let mut config = device
        .default_input_config()
        .expect("Failed to get input config");

    println!("Entering while loop based on config channels != 1, channels now is: {}", config.channels());
    while config.channels() != 2 {
        let mut supported_configs_range = device
            .supported_input_configs()
            .expect("error while querying configs");
        config = match supported_configs_range.next() {
            Some(conf) => conf.with_sample_rate(SampleRate(16000)), //16K from deepspeech
            None => break,
        };
    }
    println!("Finished while loop");
    config
}
fn get_default_device() -> Device {
    let host = cpal::default_host();
    host.default_input_device().expect("no input device found")
}
