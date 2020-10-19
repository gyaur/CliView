#![feature(proc_macro_hygiene, decl_macro)]
use lib::Config as CliViewConfig;
use lib::GenericResult as Result;
use lib::QueueState;
use lib::Url;
use lib::{extract_url, QueueStateSendable};
use rocket;
use rocket::config::{Config, Environment};
use rocket::http::Status;
use rocket::State;
use rocket_contrib::json::Json;

#[rocket::get("/front")]
fn front(state: State<QueueState>) -> Json<Option<Url>> {
    let mut queue = state.queue.lock().unwrap();
    Json(queue.pop_front())
}

#[rocket::get("/queue")]
fn queue_get(state: State<QueueState>) -> Json<QueueStateSendable> {
    Json(state.to_response())
}

#[rocket::post("/queue", data = "<data>")]
fn queue_post(state: State<QueueState>, data: Json<Url>) -> Status {
    let res = extract_url(&data.0);
    match res {
        Ok(url) => {
            let mut queue = state.queue.lock().unwrap();
            queue.push_back(url);
            Status::Accepted
        }
        Err(_) => Status::BadRequest,
    }
}
fn main() -> Result<()> {
    let cfg = CliViewConfig::load()?;
    let rocket_config = Config::build(Environment::Staging)
        .address("127.0.0.1")
        .port(cfg.queue_port)
        .workers(cfg.num_workers)
        .unwrap();

    rocket::custom(rocket_config)
        .mount("/", rocket::routes![front, queue_get, queue_post])
        .manage(QueueState::new())
        .launch();

    Ok(())
}
