use lib::Url;
use lib::{Action, Config as CliViewConfig};
use lib::{GenericResult as Result, Player, Volume};
use lib::{OMXPlayer, PlaybackStatus};
use reqwest::blocking::get;
use std::thread::sleep;
use std::time::Duration;

fn stream_loop(cfg: CliViewConfig, player: Box<dyn Player>) -> Result<()> {
    loop {
        let client = reqwest::blocking::Client::new();
        let mut volume = get(&cfg.command_volume_address)?.json::<Volume>()?;
        let curr = get(&cfg.queue_front_address)?.json::<Option<Url>>()?;
        // get next video from squeue service
        if let Some(url) = curr.clone() {
            //start process
            let mut process = player.start(url, &volume)?;
            let mut playback_status = true;
            client
                .post(&cfg.command_playback_address)
                .json(&PlaybackStatus::new(playback_status))
                .send()?;
            sleep(cfg.playback_start_timeout);
            player.pause(&mut process)?;
            sleep(cfg.playback_loadscreen_timeout);
            player.play(&mut process)?;
            loop {
                if let Some(cmd) = get(&cfg.command_front_address)?.json::<Option<Action>>()? {
                    println!("{:?}", cmd);
                    let result = player.work(
                        &mut process,
                        &mut volume,
                        &mut playback_status,
                        &cmd,
                        &client,
                        &cfg,
                    );
                    println!("{:?}", result);
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
