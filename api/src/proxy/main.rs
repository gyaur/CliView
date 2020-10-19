#![feature(proc_macro_hygiene, decl_macro)]
use lib::Config as CliViewConfig;
use lib::GenericResult as Result;
use rocket;
use rocket::config::{Config, Environment};
use rocket::response::Redirect;

#[rocket::get("/queue")]
fn queue_get() -> Redirect {
    Redirect::to(format!("http://localhost:{}/queue", 5001))
}
//TODO: Write macros to do this for every function

fn main() -> Result<()> {
    let cfg = CliViewConfig::load()?;
    let rocket_config = Config::build(Environment::Staging)
        .address("127.0.0.1")
        .port(cfg.proxy_port)
        .workers(cfg.num_workers)
        .unwrap();

    rocket::custom(rocket_config)
        .mount("/", rocket::routes![queue_get])
        .launch();

    Ok(())
}
