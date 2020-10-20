#![feature(proc_macro_hygiene, decl_macro)]
use lib::gen_proxy_function;
use lib::Config as CliViewConfig;
use lib::GenericResult as Result;
use rocket::config::{Config, Environment};
use rocket::response::Redirect;

//Queue service functions
gen_proxy_function!(queue_get, "/queue", get, 5001);
gen_proxy_function!(queue_post, "/queue", post, 5001);

//Commnad functions
//TODO: Add command functions

fn main() -> Result<()> {
    let cfg: CliViewConfig = CliViewConfig::load()?;
    let rocket_config = Config::build(Environment::Staging)
        .address("127.0.0.1")
        .port(cfg.proxy_port)
        .workers(cfg.num_workers)
        .unwrap();

    rocket::custom(rocket_config)
        .mount("/", rocket::routes![queue_get, queue_post])
        .launch();

    Ok(())
}
