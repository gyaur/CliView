#![feature(proc_macro_hygiene, decl_macro)]
use lib::Config as CliViewConfig;
use lib::Event;
use lib::GenericResult as Result;
use rocket;
use rocket::config::{Config, Environment};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    let cfg = CliViewConfig::load()?;
    let (sender, receiver) = channel::<Option<Event>>();
    let streaming_thread = thread::spawn(move || loop {
        println!("{:?}", receiver.try_recv());
        thread::sleep(Duration::from_secs(1));
    });
    let rocket_config = Config::build(Environment::Staging)
        .address("127.0.0.1")
        .port(cfg.streamer_port)
        .workers(cfg.num_workers)
        .unwrap();

    sender.send(Some(Event::Skip))?;

    rocket::custom(rocket_config)
        .mount("/", rocket::routes![])
        .launch();

    println!("duck");

    let _res = streaming_thread.join();

    Ok(())
}
