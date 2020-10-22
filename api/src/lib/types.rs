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

impl Default for Volume {
    fn default() -> Self {
        Self { volume: 0 }
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

#[derive(Debug, Clone,Serialize,Deserialize)]
pub enum Action {
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

impl Default for QueueState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandQueue {
    pub queue: Mutex<VecDeque<Action>>,
    pub volume: Mutex<Volume>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandQueueSendable {
    pub queue: Vec<Action>,
    pub volume: Volume,
}

impl CommandQueueSendable {
    pub fn new(queue: Vec<Action>, volume: Volume) -> Self {
        Self { queue, volume }
    }
}

impl CommandQueue {
    pub fn new() -> Self {
        Self {
            queue: Mutex::from(VecDeque::new()),
            volume: Mutex::from(Volume::default()),
        }
    }
    pub fn to_response(&self) -> CommandQueueSendable {
        let queue = self.queue.lock().unwrap();
        let volume = self.volume.lock().unwrap();

        CommandQueueSendable::new(
            queue.iter().map(|x| x.to_owned()).collect::<Vec<Action>>(),
            Volume::new(volume.volume),
        )
    }
}

impl Default for CommandQueue {
    fn default() -> Self {
        Self::new()
    }
}
