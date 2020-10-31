use lib::GenericResult as Result;
use lib::Url;
use lib::{stream, write_to_stdin};
use lib::{Action, Config as CliViewConfig};
// use reqwest;
use std::thread::sleep;
use std::time::Duration;

fn stream_loop(cfg: CliViewConfig) -> Result<()> {
    loop {
        // get next video from squeue service
        let queue_address: &str = &format!("http://localhost:{}/front", cfg.queue_port);
        let command_address: &str = &format!("http://localhost:{}/front", cfg.command_port);
        let curr = reqwest::blocking::get(queue_address)?.json::<Option<Url>>()?;
        if let Some(url) = curr.clone() {
            //start process
            let mut process = stream(&url, 0).unwrap();
            sleep(cfg.playback_start_timeout);
            write_to_stdin(&mut process, "p")?;
            sleep(cfg.playback_loadscreen_timeout);
            write_to_stdin(&mut process, "p")?;
            loop {
                if let Some(cmd) =
                    reqwest::blocking::get(command_address)?.json::<Option<Action>>()?
                {
                    println!("{:?}", cmd);
                }
                if let Some(_val) = process.poll() {
                    break;
                }
                sleep(cfg.command_wait_timeout);
                //reqwest command queue
                // if let Ok(command) = receiver.try_recv() {
                //     //fn to interact with the process
                // }
                //poll the process is running and try_receive from receiver
            }
        }
        sleep(Duration::from_millis(500));
    }
}

fn main() -> Result<()> {
    let cfg = CliViewConfig::load()?;
    stream_loop(cfg)?;

    Ok(())
}
