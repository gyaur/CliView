#![feature(proc_macro_hygiene, decl_macro)]
use lib::Ammount;
use lib::Url;
use lib::{Action, GenericResult as Result, Volume};
use lib::{CommandQueue, Config as CliViewConfig};
use rocket::config::{Config, Environment};
use rocket::http::Status;
use rocket::State;
use rocket_contrib::json::Json;

#[rocket::get("/front")]
fn front(state: State<CommandQueue>) -> Json<Option<Action>> {
    let mut queue = state.queue.lock().unwrap();
    Json(queue.pop_front())
}

#[rocket::post("/stream", data = "<url>")]
fn stream(state: State<CommandQueue>, url: Json<Url>) {
    let mut queue = state.queue.lock().unwrap();
    queue.push_back(Action::Stream(url.0));
}

#[rocket::post("/inc")]
fn increase_volume(state: State<CommandQueue>) -> Status {
    let mut queue = state.queue.lock().unwrap();
    let mut volume = state.volume.lock().unwrap();
    if volume.volume > 9 {
        return Status::BadRequest;
    }
    volume.volume += 1;
    queue.push_back(Action::VolumeUp);
    Status::Ok
}

#[rocket::post("/dec")]
fn lower_volume(state: State<CommandQueue>) -> Status {
    let mut queue = state.queue.lock().unwrap();
    let mut volume = state.volume.lock().unwrap();
    if volume.volume < 1 {
        return Status::BadRequest;
    }
    volume.volume -= 1;
    queue.push_back(Action::VolumeDown);
    Status::Ok
}

#[rocket::post("/volume", data = "<incoming_volume>")]
fn set_volume(state: State<CommandQueue>, incoming_volume: Json<Volume>) -> Status {
    let mut queue = state.queue.lock().unwrap();
    let mut volume = state.volume.lock().unwrap();
    if !(0..=10).contains(&incoming_volume.volume) {
        return Status::BadRequest;
    }
    volume.volume = incoming_volume.volume;
    queue.push_back(Action::VolumeSet(incoming_volume.0));
    Status::Ok
}

#[rocket::get("/volume")]
fn get_volume(state: State<CommandQueue>) -> Json<Volume> {
    Json(state.to_response().volume)
}

#[rocket::post("/seek", data = "<seek>")]
fn seek(state: State<CommandQueue>, seek: Json<Ammount>) -> Status {
    let mut queue = state.queue.lock().unwrap();
    //TODO: Add function to actually check
    if seek.ammount == 0 {
        return Status::BadRequest;
    }
    queue.push_back(Action::Seek(seek.0));
    Status::Ok
}

#[rocket::post("/skip")]
fn skip(state: State<CommandQueue>) {
    let mut queue = state.queue.lock().unwrap();
    queue.push_back(Action::Skip);
}

fn setup_rocket(cfg: CliViewConfig) -> rocket::Rocket {
    let rocket_config = Config::build(Environment::Development)
        .address("0.0.0.0")
        .port(cfg.command_port)
        .workers(cfg.num_workers)
        .finalize()
        .unwrap();

    rocket::custom(rocket_config)
        .mount(
            "/",
            rocket::routes![
                lower_volume,
                increase_volume,
                set_volume,
                get_volume,
                stream,
                seek,
                skip,
                front
            ],
        )
        .manage(CommandQueue::new())
}
fn main() -> Result<()> {
    let cfg = CliViewConfig::load()?;
    let rocket = setup_rocket(cfg);
    rocket.launch();

    Ok(())
}
