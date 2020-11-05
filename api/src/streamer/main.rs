use lib::OMXPlayer;
use lib::Url;
use lib::{Action, Config as CliViewConfig};
use lib::{GenericResult as Result, Player, Volume};
// use reqwest;
use std::thread::sleep;
use std::time::Duration;

fn stream_loop(cfg: CliViewConfig, player: Box<dyn Player>) -> Result<()> {
    loop {
        // get next video from squeue service
        let queue_address: &str = &format!("http://localhost:{}/front", cfg.queue_port);
        let command_address: &str = &format!("http://localhost:{}/front", cfg.command_port);
        let volume_address: &str = &format!("http://localhost:{}/volume", cfg.command_port);
        let curr = reqwest::blocking::get(queue_address)?.json::<Option<Url>>()?;
        let volume = reqwest::blocking::get(volume_address)?.json::<Volume>()?;
        if let Some(url) = curr.clone() {
            //start process
            let mut process = player.start(&url, &volume)?;
            sleep(cfg.playback_start_timeout);
            player.pause(&mut process);
            sleep(cfg.playback_loadscreen_timeout);
            player.play(&mut process);
            loop {
                if let Some(cmd) =
                    reqwest::blocking::get(command_address)?.json::<Option<Action>>()?
                {
                    match cmd {
                        Action::Skip => player.skip(&mut process),
                        Action::Seek(ammount) => player.seek(&mut process, ammount),
                        Action::Stream(url) => {
                            player.stop(&mut process);
                            process = player.start(&url, &volume)?;
                            sleep(cfg.playback_start_timeout);
                            player.pause(&mut process);
                            sleep(cfg.playback_loadscreen_timeout);
                            player.play(&mut process);
                        }
                        Action::VolumeDown => player.decrease_volume(&mut process),
                        Action::VolumeUp => player.increase_volume(&mut process),
                        Action::VolumeSet(vol) => player.set_volume(&mut process, vol),
                    }
                }
                if let Some(_val) = process.poll() {
                    break;
                }
                sleep(cfg.command_wait_timeout);
            }
        }
        sleep(Duration::from_millis(500));
    }
}

fn main() -> Result<()> {
    let cfg = CliViewConfig::load()?;
    let player = OMXPlayer;
    stream_loop(cfg, Box::from(player))?;

    Ok(())
}
