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
    fn set_volume(&self, process: &mut Popen, volume: Volume, current_volume: &Volume);
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

    fn set_volume(&self, mut process: &mut Popen, volume: Volume, current_volume: &Volume) {
        let delta = current_volume.volume - volume.volume;
        for _ in 0..delta.abs() {
            if delta > 0 {
                self.decrease_volume(&mut process);
            } else {
                self.increase_volume(&mut process);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use subprocess::{Popen, PopenConfig, Redirection};

    use crate::{write_to_stdin, Ammount, Player, Url, Volume};

    struct TestPlayer;
    impl Player for TestPlayer {
        fn start(
            &self,
            media: &crate::Url,
            volume: &crate::Volume,
        ) -> crate::GenericResult<subprocess::Popen> {
            Ok(Popen::create(
                &["cat"],
                PopenConfig {
                    stdin: Redirection::Pipe,
                    stdout: Redirection::Pipe,
                    stderr: Redirection::None,
                    ..Default::default()
                },
            )?)
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

        fn set_volume(&self, mut process: &mut Popen, volume: Volume, current_volume: &Volume) {
            let delta = current_volume.volume - volume.volume;
            for _ in 0..delta.abs() {
                if delta > 0 {
                    self.decrease_volume(&mut process);
                } else {
                    self.increase_volume(&mut process);
                }
            }
        }
    }

    fn init_player() -> (Popen, TestPlayer) {
        let player = TestPlayer;
        let result = player
            .start(&Url::new(String::from("test")), &Volume::new(5))
            .unwrap();
        return (result, player);
    }

    #[test]
    fn test_start() {
        let (mut process, _player) = init_player();
        let (stdout, _) = process.communicate(Some("")).unwrap();
        assert!(stdout.is_some());
        assert!(stdout.unwrap().is_empty());
    }

    #[test]
    fn test_stop() {
        let (mut process, player) = init_player();
        player.stop(&mut process);
        let (stdout, _) = process.communicate(Some("")).unwrap();
        assert!(stdout.is_some());
        assert!(stdout.unwrap() == String::from("q"));
    }

    #[test]
    fn test_pause() {
        let (mut process, player) = init_player();
        player.pause(&mut process);
        let (stdout, _) = process.communicate(Some("")).unwrap();
        assert!(stdout.is_some());
        assert!(stdout.unwrap() == String::from("p"));
    }

    #[test]
    fn test_play() {
        let (mut process, player) = init_player();
        player.play(&mut process);
        let (stdout, _) = process.communicate(Some("")).unwrap();
        assert!(stdout.is_some());
        assert!(stdout.unwrap() == String::from("p"));
    }

    #[test]
    fn test_skip() {
        let (mut process, player) = init_player();
        player.skip(&mut process);
        let (stdout, _) = process.communicate(Some("")).unwrap();
        assert!(stdout.is_some());
        assert!(stdout.unwrap() == String::from("q"));
    }

    #[test]
    fn test_seek_30() {
        let (mut process, player) = init_player();
        player.seek(&mut process, Ammount::new(30));
        let (stdout, _) = process.communicate(Some("")).unwrap();
        assert!(stdout.is_some());
        assert!(stdout.unwrap() == String::from("\x1b\x5b\x43"));
    }

    #[test]
    fn test_seek_minus_30() {
        let (mut process, player) = init_player();
        player.seek(&mut process, Ammount::new(-30));
        let (stdout, _) = process.communicate(Some("")).unwrap();
        assert!(stdout.is_some());
        assert!(stdout.unwrap() == String::from("\x1b\x5b\x44"));
    }

    #[test]
    fn test_seek_60() {
        let (mut process, player) = init_player();
        player.seek(&mut process, Ammount::new(60));
        let (stdout, _) = process.communicate(Some("")).unwrap();
        assert!(stdout.is_some());
        assert!(stdout.unwrap() == String::from("\x1b\x5b\x43\x1b\x5b\x43"));
    }

    #[test]
    fn test_seek_600() {
        let (mut process, player) = init_player();
        player.seek(&mut process, Ammount::new(600));
        let (stdout, _) = process.communicate(Some("")).unwrap();
        assert!(stdout.is_some());
        assert!(stdout.unwrap() == String::from("\x1b\x5b\x41"));
    }

    #[test]
    fn test_seek_minus_600() {
        let (mut process, player) = init_player();
        player.seek(&mut process, Ammount::new(-600));
        let (stdout, _) = process.communicate(Some("")).unwrap();
        assert!(stdout.is_some());
        assert!(stdout.unwrap() == String::from("\x1b\x5b\x42"));
    }

    #[test]
    fn test_inc_volume() {
        let (mut process, player) = init_player();
        player.increase_volume(&mut process);
        let (stdout, _) = process.communicate(Some("")).unwrap();
        assert!(stdout.is_some());
        assert!(stdout.unwrap() == String::from("+"));
    }

    #[test]
    fn test_dec_volume() {
        let (mut process, player) = init_player();
        player.decrease_volume(&mut process);
        let (stdout, _) = process.communicate(Some("")).unwrap();
        assert!(stdout.is_some());
        assert!(stdout.unwrap() == String::from("-"));
    }

    #[test]
    fn test_positive_set_volume() {
        let (mut process, player) = init_player();
        player.set_volume(&mut process, Volume::new(4), &Volume::new(0));
        let (stdout, _) = process.communicate(Some("")).unwrap();
        assert!(stdout.is_some());
        assert!(stdout.unwrap() == String::from("++++"));
    }

    #[test]
    fn test_negative_set_volume() {
        let (mut process, player) = init_player();
        player.set_volume(&mut process, Volume::new(0), &Volume::new(4));
        let (stdout, _) = process.communicate(Some("")).unwrap();
        assert!(stdout.is_some());
        assert!(stdout.unwrap() == String::from("----"));
    }
}
