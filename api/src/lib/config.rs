use confy;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub proxy_port: u16,
    pub queue_port: u16,
    pub streamer_port: u16,
    pub num_workers: u16,
}

/// `Config` implements `Default`
impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            proxy_port: 5000,
            queue_port: 5001,
            streamer_port: 5002,
            num_workers: 4,
        }
    }
}

impl Config {
    pub fn load() -> Result<Config, confy::ConfyError> {
        confy::load("cliview")
    }
}
