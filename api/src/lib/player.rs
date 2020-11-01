use crate::{stream, Ammount, Url, Volume};
use crate::{write_to_stdin, GenericResult as Result};
use subprocess::Popen;

pub trait Player {
    fn start(&self, media: &Url, volume: &Volume) -> Result<Popen>;
    fn stop(&self, process: &mut Popen);
    fn play(&self, process: &mut Popen);
    fn pause(&self, process: &mut Popen);
    fn skip(&self, process: &mut Popen);
    fn seek(&self, process: &mut Popen, ammount: Ammount);
    fn increase_volume(&self, process: &mut Popen);
    fn decrease_volume(&self, process: &mut Popen);
    fn set_volume(&self, process: &mut Popen, volume: Volume);
}

pub struct OMXPlayer;

impl Player for OMXPlayer {
    fn start(&self, media: &Url, volume: &Volume) -> Result<Popen> {
        stream(&media, volume.volume)
    }

    fn stop(&self, process: &mut Popen) {
        write_to_stdin(process, "q");
    }

    fn play(&self, process: &mut Popen) {
        write_to_stdin(process, "p");
    }

    fn pause(&self, process: &mut Popen) {
        write_to_stdin(process, "p");
    }

    fn skip(&self, process: &mut Popen) {
        write_to_stdin(process, "q");
    }

    fn seek(&self, process: &mut Popen, ammount: Ammount) {
        for amm in ammount.as_vec_of_seconds() {
            match amm {
                30 => write_to_stdin(process, "\x1b\x5b\x43"),
                -30 => write_to_stdin(process, "\x1b\x5b\x44"),
                600 => write_to_stdin(process, "\x1b\x5b\x41"),
                -600 => write_to_stdin(process, "\x1b\x5b\x42"),
                _ => panic!("something has gone increadibly wrong"),
            };
        }
    }

    fn increase_volume(&self, process: &mut Popen) {
        write_to_stdin(process, "+");
    }

    fn decrease_volume(&self, process: &mut Popen) {
        write_to_stdin(process, "-");
    }

    fn set_volume(&self, process: &mut Popen, volume: Volume) {
        todo!()
    }
}
