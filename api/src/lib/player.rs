use std::{thread::sleep, time::Duration};

use crate::{stream, Action, Ammount, Config, PlaybackStatus, Url, Volume};
use crate::{write_to_stdin, GenericResult as Result};
use reqwest::blocking::Client;
use subprocess::Popen;

pub trait Player {
    fn start(&self, media: Url, volume: &Volume) -> Result<Popen>;
    fn stop(&self, process: &mut Popen) -> Result<()>;
    fn play(&self, process: &mut Popen) -> Result<()>;
    fn pause(&self, process: &mut Popen) -> Result<()>;
    fn skip(&self, process: &mut Popen) -> Result<()>;
    fn seek(&self, process: &mut Popen, ammount: Ammount) -> Result<()>;
    fn increase_volume(&self, process: &mut Popen) -> Result<()>;
    fn decrease_volume(&self, process: &mut Popen) -> Result<()>;
    fn set_volume(
        &self,
        process: &mut Popen,
        volume: &Volume,
        current_volume: &mut Volume,
    ) -> Result<()>;
    fn work(
        &self,
        process: &mut Popen,
        current_volume: &mut Volume,
        playback_status: &mut bool,
        cmd: &Action,
        client: &Client,
        cfg: &Config,
        playback_address: &str,
    ) -> Result<()>;
}

pub struct OMXPlayer;

impl Player for OMXPlayer {
    fn start(&self, media: Url, volume: &Volume) -> Result<Popen> {
        stream(media, *volume)
    }

    fn stop(&self, process: &mut Popen) -> Result<()> {
        write_to_stdin(process, "q")
    }

    fn play(&self, process: &mut Popen) -> Result<()> {
        write_to_stdin(process, "p")
    }

    fn pause(&self, process: &mut Popen) -> Result<()> {
        write_to_stdin(process, "p")
    }

    fn skip(&self, process: &mut Popen) -> Result<()> {
        write_to_stdin(process, "q")
    }

    fn seek(&self, process: &mut Popen, ammount: Ammount) -> Result<()> {
        for amm in ammount.as_vec_of_seconds() {
            match amm {
                30 => write_to_stdin(process, "\x1b\x5b\x43")?,
                -30 => write_to_stdin(process, "\x1b\x5b\x44")?,
                600 => write_to_stdin(process, "\x1b\x5b\x41")?,
                -600 => write_to_stdin(process, "\x1b\x5b\x42")?,
                _ => panic!("something has gone increadibly wrong"),
            };
            sleep(Duration::from_millis(50));
        }
        Ok(())
    }

    fn increase_volume(&self, process: &mut Popen) -> Result<()> {
        write_to_stdin(process, "+")
    }

    fn decrease_volume(&self, process: &mut Popen) -> Result<()> {
        write_to_stdin(process, "-")
    }

    fn set_volume(
        &self,
        mut process: &mut Popen,
        volume: &Volume,
        current_volume: &mut Volume,
    ) -> Result<()> {
        let delta = current_volume.volume - volume.volume;
        *current_volume = *volume;
        println!("{:?}", delta);
        for i in 0..delta.abs() {
            println!("{:?}", i);
            if delta > 0 {
                self.decrease_volume(&mut process)?;
            } else {
                self.increase_volume(&mut process)?;
            }
            sleep(Duration::from_millis(50));
        }
        Ok(())
    }

    fn work(
        &self,
        mut process: &mut Popen,
        mut current_volume: &mut Volume,
        playback_status: &mut bool,
        cmd: &Action,
        client: &Client,
        cfg: &Config,
        playback_address: &str,
    ) -> Result<()> {
        match cmd {
            Action::Skip => self.skip(&mut process),
            Action::Seek(ammount) => self.seek(&mut process, *ammount),
            Action::Stream(url) => {
                self.stop(&mut process)?;
                *process = self.start(url.clone(), &current_volume)?;
                sleep(cfg.playback_start_timeout);
                self.pause(&mut process)?;
                sleep(cfg.playback_loadscreen_timeout);
                self.play(&mut process)?;
                Ok(())
            }
            Action::VolumeDown => {
                self.decrease_volume(&mut process)?;
                current_volume.volume -= 1;
                Ok(())
            }
            Action::VolumeUp => {
                self.increase_volume(&mut process)?;
                current_volume.volume += 1;
                Ok(())
            }
            Action::VolumeSet(vol) => self.set_volume(&mut process, &vol, &mut current_volume),
            Action::Play => {
                if !*playback_status {
                    self.pause(&mut process)?;
                    *playback_status = true;
                    client
                        .post(playback_address)
                        .json(&PlaybackStatus::new(*playback_status))
                        .send()?;
                }
                Ok(())
            }
            Action::Pause => {
                if *playback_status {
                    self.pause(&mut process)?;
                    *playback_status = false;
                    client
                        .post(playback_address)
                        .json(&PlaybackStatus::new(*playback_status))
                        .send()?;
                }
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use subprocess::{Popen, PopenConfig, Redirection};

    use crate::{Ammount, OMXPlayer, Player, Volume};

    fn init_player() -> Popen {
        let result = Popen::create(
            &["cat"],
            PopenConfig {
                stdin: Redirection::Pipe,
                stdout: Redirection::Pipe,
                stderr: Redirection::None,
                ..Default::default()
            },
        )
        .unwrap();

        result
    }

    #[test]
    fn test_start() {
        let mut process = init_player();
        let (stdout, _) = process.communicate(Some("")).unwrap();
        assert!(stdout.is_some());
        assert!(stdout.unwrap().is_empty());
    }

    #[test]
    fn test_stop() {
        let mut process = init_player();
        OMXPlayer.stop(&mut process).unwrap();
        let (stdout, _) = process.communicate(Some("")).unwrap();
        assert!(stdout.is_some());
        assert!(stdout.unwrap() == String::from("q"));
    }

    #[test]
    fn test_pause() {
        let mut process = init_player();
        OMXPlayer.pause(&mut process).unwrap();
        let (stdout, _) = process.communicate(Some("")).unwrap();
        assert!(stdout.is_some());
        assert!(stdout.unwrap() == String::from("p"));
    }

    #[test]
    fn test_play() {
        let mut process = init_player();
        OMXPlayer.play(&mut process).unwrap();
        let (stdout, _) = process.communicate(Some("")).unwrap();
        assert!(stdout.is_some());
        assert!(stdout.unwrap() == String::from("p"));
    }

    #[test]
    fn test_skip() {
        let mut process = init_player();
        OMXPlayer.skip(&mut process).unwrap();
        let (stdout, _) = process.communicate(Some("")).unwrap();
        assert!(stdout.is_some());
        assert!(stdout.unwrap() == String::from("q"));
    }

    #[test]
    fn test_seek_30() {
        let mut process = init_player();
        OMXPlayer.seek(&mut process, Ammount::new(30)).unwrap();
        let (stdout, _) = process.communicate(Some("")).unwrap();
        assert!(stdout.is_some());
        assert!(stdout.unwrap() == String::from("\x1b\x5b\x43"));
    }

    #[test]
    fn test_seek_minus_30() {
        let mut process = init_player();
        OMXPlayer.seek(&mut process, Ammount::new(-30)).unwrap();
        let (stdout, _) = process.communicate(Some("")).unwrap();
        assert!(stdout.is_some());
        assert!(stdout.unwrap() == String::from("\x1b\x5b\x44"));
    }

    #[test]
    fn test_seek_60() {
        let mut process = init_player();
        OMXPlayer.seek(&mut process, Ammount::new(60)).unwrap();
        let (stdout, _) = process.communicate(Some("")).unwrap();
        assert!(stdout.is_some());
        assert!(stdout.unwrap() == String::from("\x1b\x5b\x43\x1b\x5b\x43"));
    }

    #[test]
    fn test_seek_600() {
        let mut process = init_player();
        OMXPlayer.seek(&mut process, Ammount::new(600)).unwrap();
        let (stdout, _) = process.communicate(Some("")).unwrap();
        assert!(stdout.is_some());
        assert!(stdout.unwrap() == String::from("\x1b\x5b\x41"));
    }

    #[test]
    fn test_seek_minus_600() {
        let mut process = init_player();
        OMXPlayer.seek(&mut process, Ammount::new(-600)).unwrap();
        let (stdout, _) = process.communicate(Some("")).unwrap();
        assert!(stdout.is_some());
        assert!(stdout.unwrap() == String::from("\x1b\x5b\x42"));
    }

    #[test]
    fn test_inc_volume() {
        let mut process = init_player();
        OMXPlayer.increase_volume(&mut process).unwrap();
        let (stdout, _) = process.communicate(Some("")).unwrap();
        assert!(stdout.is_some());
        assert!(stdout.unwrap() == String::from("+"));
    }

    #[test]
    fn test_dec_volume() {
        let mut process = init_player();
        OMXPlayer.decrease_volume(&mut process).unwrap();
        let (stdout, _) = process.communicate(Some("")).unwrap();
        assert!(stdout.is_some());
        assert!(stdout.unwrap() == String::from("-"));
    }

    #[test]
    fn test_positive_set_volume() {
        let mut process = init_player();
        OMXPlayer
            .set_volume(&mut process, &Volume::new(4), &mut Volume::new(0))
            .unwrap();
        let (stdout, _) = process.communicate(Some("")).unwrap();
        assert!(stdout.is_some());
        assert!(stdout.unwrap() == String::from("++++"));
    }

    #[test]
    fn test_negative_set_volume() {
        let mut process = init_player();
        OMXPlayer
            .set_volume(&mut process, &Volume::new(0), &mut Volume::new(4))
            .unwrap();
        let (stdout, _) = process.communicate(Some("")).unwrap();
        assert!(stdout.is_some());
        assert!(stdout.unwrap() == String::from("----"));
    }
}
