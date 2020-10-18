use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, sync::Mutex};

pub type Error = Box<dyn std::error::Error>;
pub type GenericResult<T> = Result<T, Error>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Url {
    pub url: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Volume {
    pub volume: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ammount {
    pub ammount: i32,
}

#[derive(Debug, Clone)]
pub enum Event {
    Stream(Url),
    Skip,
    VolumeUp,
    VolumeDown,
    VolumeSet(Volume),
    Seek(Ammount),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueueState {
    pub queue: Mutex<VecDeque<Url>>,
}

impl QueueState {
    pub fn new() -> Self {
        Self {
            queue: Mutex::from(VecDeque::new()),
        }
    }
}
