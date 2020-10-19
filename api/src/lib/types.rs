use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, sync::Mutex};

pub type Error = Box<dyn std::error::Error>;
pub type GenericResult<T> = Result<T, Error>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Url {
    pub url: String,
}

impl Url {
    pub fn new(url: String) -> Self {
        Self { url }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Volume {
    pub volume: i32,
}

impl Volume {
    pub fn new(volume: i32) -> Self {
        Self { volume }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ammount {
    pub ammount: i32,
}

impl Ammount {
    pub fn new(ammount: i32) -> Self {
        Self { ammount }
    }
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

#[derive(Debug, Serialize, Deserialize)]
pub struct QueueStateSendable {
    pub queue: Vec<Url>,
}

impl QueueStateSendable {
    pub fn new(queue: Vec<Url>) -> Self {
        Self { queue }
    }
}

impl QueueState {
    pub fn new() -> Self {
        Self {
            queue: Mutex::from(VecDeque::new()),
        }
    }
    pub fn to_response(&self) -> QueueStateSendable {
        let queue = self.queue.lock().unwrap();

        QueueStateSendable::new(queue.iter().map(|x| x.to_owned()).collect::<Vec<Url>>())
    }
}
