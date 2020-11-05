use rustorm::*;
use rustorm::{FromDao, ToColumnNames, ToDao, ToTableName};
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, sync::Mutex};

pub type Error = Box<dyn std::error::Error>;
pub type GenericResult<T> = Result<T, Error>;

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, ToDao, ToColumnNames, ToTableName, FromDao,
)]
pub struct Url {
    pub url: String,
}

#[derive(Debug, FromDao, ToColumnNames, ToTableName)]
pub struct RetriveUrl {
    pub id: i32,
    pub url: String,
}

impl Url {
    pub fn new(url: String) -> Self {
        Self { url }
    }
}

impl From<String> for Url {
    fn from(inc: String) -> Self {
        Url { url: inc }
    }
}

impl From<RetriveUrl> for Url {
    fn from(inc: RetriveUrl) -> Self {
        Url::from(inc.url)
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

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub struct Ammount {
    pub ammount: i32,
}

impl Ammount {
    pub fn new(ammount: i32) -> Self {
        Self { ammount }
    }

    pub fn as_vec_of_seconds(self) -> impl Iterator<Item = i32> {
        let positve = self.ammount > 0;
        let mut abs_ammount = self.ammount.abs();
        let num_big = abs_ammount / 600;
        abs_ammount -= num_big * 600;
        let num_small = abs_ammount / 30;
        std::iter::repeat(if positve { 600 } else { -600 })
            .take(num_big as usize)
            .chain(std::iter::repeat(if positve { 30 } else { -30 }).take(num_small as usize))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl From<Vec<Url>> for QueueState {
    fn from(urls: Vec<Url>) -> Self {
        let queue: Mutex<VecDeque<Url>> = Mutex::new(urls.into_iter().collect());
        QueueState { queue }
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

#[cfg(test)]
mod test {
    use crate::Ammount;
    #[test]
    fn test_ammount_as_vec_of_seconds() {
        let test_ammounts = vec![
            Ammount::new(0),
            Ammount::new(30),
            Ammount::new(120),
            Ammount::new(570),
            Ammount::new(1230),
            Ammount::new(-30),
            Ammount::new(-120),
            Ammount::new(-570),
            Ammount::new(-1230),
        ];

        //Check if the sums are correct
        assert!(test_ammounts
            .iter()
            .all(|x| x.as_vec_of_seconds().sum::<i32>() == x.ammount));

        //Check wether the 30s add up to 600
        assert!(test_ammounts.iter().all(|x| x
            .as_vec_of_seconds()
            .filter(|y| y.abs() == 30)
            .sum::<i32>()
            < 600));
    }
}
