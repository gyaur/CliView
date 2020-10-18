use confy;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub proxy_port: u16,
    pub queue_port: u16,
    pub streamer_port: u16,
    pub command_port: u16,
    pub num_workers: u16,
    pub playback_start_timeout: Duration,
    pub playback_loadscreen_timeout: Duration,
    pub command_wait_timeout: Duration,
}

/// `Config` implements `Default`
impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            proxy_port: 5000,
            queue_port: 5001,
            streamer_port: 5002,
            command_port: 5003,
            num_workers: 4,
            playback_start_timeout: Duration::from_millis(500),
            playback_loadscreen_timeout: Duration::from_secs(3),
            command_wait_timeout: Duration::from_millis(100),
        }
    }
}

impl Config {
    pub fn load() -> Result<Config, confy::ConfyError> {
        confy::load("cliview")
    }
}
