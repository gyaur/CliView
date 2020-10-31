use subprocess::Popen;
use crate::{Ammount, Url, Volume};

pub trait Player {
    fn start(process: Popen, media: Url, volume: Volume);
    fn stop(process: Popen);
    fn play(process: Popen);
    fn pause(process: Popen);
    fn skip(process: Popen);
    fn seek(process: Popen, ammount: Ammount);
    fn increase_volume(process: Popen);
    fn decrease_volume(process: Popen);
    fn set_volume(process: Popen, volume: Volume);
}
