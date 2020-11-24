use regex;
#[cfg(feature = "db")]
use rustorm::*;
#[cfg(feature = "db")]
use rustorm::{FromDao, ToColumnNames, ToDao, ToTableName};
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, sync::Mutex};

pub type Error = Box<dyn std::error::Error>;
pub type GenericResult<T> = Result<T, Error>;

#[cfg(feature = "db")]
#[derive(
    Debug, Clone, Serialize, Deserialize, ToDao, ToColumnNames, ToTableName, FromDao, PartialEq,
)]
pub struct Url {
    pub url: String,
}
#[cfg(not(feature = "db"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Url {
    pub url: String,
}
#[cfg(feature = "db")]
#[derive(Debug, FromDao, ToColumnNames, ToTableName)]
pub struct RetriveUrl {
    pub id: i32,
    pub url: String,
}

impl Url {
    pub fn new(url: String) -> Self {
        Self { url }
    }
    pub fn is_ip(&self) -> bool {
        let re = regex::Regex::new(r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}:\d{1,5}").unwrap();
        re.is_match(self.url.as_str())
    }
}

impl From<String> for Url {
    fn from(inc: String) -> Self {
        Url { url: inc }
    }
}

#[cfg(feature = "db")]
impl From<RetriveUrl> for Url {
    fn from(inc: RetriveUrl) -> Self {
        Url::from(inc.url)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub struct Volume {
    pub volume: i32,
}

impl Volume {
    pub fn new(volume: i32) -> Self {
        Self { volume }
    }

    pub fn as_milibells(&self) -> i32 {
        (self.volume - 10) * 300
    }
}

impl Default for Volume {
    fn default() -> Self {
        Self { volume: 5 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Action {
    Stream(Url),
    Skip,
    VolumeUp,
    VolumeDown,
    VolumeSet(Volume),
    Seek(Ammount),
    Play,
    Pause,
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
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct PlaybackStatus {
    pub status: bool,
}

impl PlaybackStatus {
    pub fn new(status: bool) -> Self {
        Self { status }
    }
}

impl Default for PlaybackStatus {
    fn default() -> Self {
        Self::new(false)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandQueue {
    pub queue: Mutex<VecDeque<Action>>,
    pub volume: Mutex<Volume>,
    pub playback_state: Mutex<PlaybackStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandQueueSendable {
    pub queue: Vec<Action>,
    pub volume: Volume,
    pub playback_state: PlaybackStatus,
}

impl CommandQueueSendable {
    pub fn new(queue: Vec<Action>, volume: Volume, playback_state: PlaybackStatus) -> Self {
        Self {
            queue,
            volume,
            playback_state,
        }
    }
}

impl CommandQueue {
    pub fn new() -> Self {
        Self {
            queue: Mutex::from(VecDeque::new()),
            volume: Mutex::from(Volume::default()),
            playback_state: Mutex::from(PlaybackStatus::default()),
        }
    }
    pub fn to_response(&self) -> CommandQueueSendable {
        let queue = self.queue.lock().unwrap();
        let volume = self.volume.lock().unwrap();
        let playback_state = self.playback_state.lock().unwrap();

        CommandQueueSendable::new(
            queue.iter().map(|x| x.to_owned()).collect::<Vec<Action>>(),
            Volume::new(volume.volume),
            PlaybackStatus::new(playback_state.status),
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
