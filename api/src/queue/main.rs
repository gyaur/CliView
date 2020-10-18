#![feature(proc_macro_hygiene, decl_macro)]
use lib::extract_url;
use lib::Config as CliViewConfig;
use lib::GenericResult as Result;
use lib::QueueState;
use lib::Url;
use rocket;
use rocket::config::{Config, Environment};
use rocket::State;
use rocket_contrib::json::Json;

#[rocket::get("/front")]
fn front(state: State<QueueState>) -> Json<Option<Url>> {
    let mut queue = state.queue.lock().unwrap();
    Json(queue.pop_front())
}

fn main() -> Result<()> {
    let cfg = CliViewConfig::load()?;
    let rocket_config = Config::build(Environment::Staging)
        .address("127.0.0.1")
        .port(cfg.queue_port)
        .workers(cfg.num_workers)
        .unwrap();

    let state = QueueState::new();
    {
        let mut queue = state.queue.lock().unwrap();
        queue.push_back(Url {
            url: extract_url("https://www.youtube.com/watch?v=9em32dDnTck")?,
        });
    }
    rocket::custom(rocket_config)
        .mount("/", rocket::routes![front])
        .manage(state)
        .launch();

    Ok(())
}
