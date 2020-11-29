#![feature(proc_macro_hygiene, decl_macro)]
use lib::gen_proxy_function;
use lib::Config as CliViewConfig;
use lib::GenericResult as Result;
use rocket::response::Redirect;
use rocket::State;
use rocket::{
    config::{Config, Environment},
    Rocket,
};

//Queue service functions
gen_proxy_function!(queue_get, "/queue", get, queue_port);
gen_proxy_function!(queue_post, "/queue", post, queue_port);
gen_proxy_function!(current_get, "/current", get, queue_port);

//Command service functions
gen_proxy_function!(stream, "/stream", post, command_port);
gen_proxy_function!(pause, "/pause", post, command_port);
gen_proxy_function!(play, "/play", post, command_port);
gen_proxy_function!(playback, "/playback", get, command_port);
gen_proxy_function!(set_playback, "/playback", post, command_port);
gen_proxy_function!(increase_volume, "/inc", post, command_port);
gen_proxy_function!(lower_volume, "/dec", post, command_port);
gen_proxy_function!(set_volume, "/volume", post, command_port);
gen_proxy_function!(get_volume, "/volume", get, command_port);
gen_proxy_function!(seek, "/seek", post, command_port);
gen_proxy_function!(skip, "/skip", post, command_port);

fn setup_rocket() -> Result<Rocket> {
    let cfg: CliViewConfig = CliViewConfig::load()?;
    let rocket_config = Config::build(Environment::Development)
        .address("0.0.0.0")
        .port(cfg.proxy_port)
        .workers(cfg.num_workers)
        .finalize()
        .unwrap();

    let cors = rocket_cors::CorsOptions::default().to_cors().unwrap();

    let rocket = rocket::custom(rocket_config)
        .mount(
            "/",
            rocket::routes![
                queue_get,
                queue_post,
                stream,
                increase_volume,
                lower_volume,
                set_volume,
                get_volume,
                seek,
                skip,
                play,
                pause,
                playback,
                set_playback
            ],
        )
        .attach(cors)
        .manage(cfg);

    Ok(rocket)
}

fn main() -> Result<()> {
    let rocket = setup_rocket()?;
    rocket.launch();
    Ok(())
}

#[cfg(test)]
mod test {
    use lib::Config as CliViewConfig;
    use rocket::{http::Status, local::Client};

    use super::setup_rocket;

    fn setup_rocket_test_client() -> Client {
        Client::new(setup_rocket().unwrap()).expect("valid rocket instance")
    }

    macro_rules! generate_test {
        ($name:ident,$port:ident,$method:ident,$uri:expr) => {
            #[test]
            fn $name() {
                let cfg: CliViewConfig = CliViewConfig::load().unwrap();
                let client = setup_rocket_test_client();
                let response = client.$method($uri).dispatch();

                assert_eq!(response.status(), Status::TemporaryRedirect);
                assert!(
                    response.headers().get("Location").nth(0).unwrap()
                        == format!("http://localhost:{}{}", cfg.$port, $uri)
                )
            }
        };
    }

    //Queue service tests
    generate_test!(test_queue_get, queue_port, get, "/queue");
    generate_test!(test_current_get, queue_port, get, "/current");
    generate_test!(test_queue_post, queue_port, post, "/queue");

    //Command service tests
    generate_test!(test_stream, command_port, post, "/stream");
    generate_test!(test_play, command_port, post, "/play");
    generate_test!(test_pause, command_port, post, "/pause");
    generate_test!(test_playback, command_port, get, "/playback");
    generate_test!(test_set_playback, command_port, post, "/playback");
    generate_test!(test_increase_volume, command_port, post, "/inc");
    generate_test!(test_lower_volume, command_port, post, "/dec");
    generate_test!(test_volume_post, command_port, post, "/volume");
    generate_test!(test_volume_get, command_port, get, "/volume");
    generate_test!(test_seek, command_port, post, "/seek");
    generate_test!(test_skip, command_port, post, "/skip");
}
