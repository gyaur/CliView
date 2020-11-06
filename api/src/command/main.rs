#![feature(proc_macro_hygiene, decl_macro)]
use lib::{extract_url, Url};
use lib::{Action, GenericResult as Result, Volume};
use lib::{Ammount, PlaybackStatus};
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

#[rocket::post("/play")]
fn play(state: State<CommandQueue>) {
    let mut queue = state.queue.lock().unwrap();
    queue.push_front(Action::Play);
}

#[rocket::post("/pause")]
fn pause(state: State<CommandQueue>) {
    let mut queue = state.queue.lock().unwrap();
    queue.push_front(Action::Pause);
}

#[rocket::get("/playback")]
fn playback(state: State<CommandQueue>) -> Json<PlaybackStatus> {
    let playback_status = state.playback_state.lock().unwrap();
    Json(PlaybackStatus::new(playback_status.status))
}

#[rocket::post("/stream", data = "<url>")]
fn stream(state: State<CommandQueue>, url: Json<Url>) {
    let mut queue = state.queue.lock().unwrap();
    let url = extract_url(&url.0).unwrap();
    queue.push_front(Action::Stream(url));
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
    if (seek.ammount % 600) % 30 != 0 {
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
                front,
                play,
                pause,
                playback
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

#[cfg(test)]
mod test {
    use crate::setup_rocket;
    use crate::CliViewConfig;
    use lib::{Action, PlaybackStatus, Volume};
    use rocket::http::ContentType;
    use rocket::http::Status;
    use rocket::local::Client;

    fn setup_rocket_test_client() -> Client {
        let cfg = CliViewConfig::load().unwrap();
        Client::new(setup_rocket(cfg)).expect("valid rocket instance")
    }
    #[test]
    fn test_front() {
        let client = setup_rocket_test_client();
        let mut response = client.get("/front").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
        let action =
            serde_json::from_str::<Option<Action>>(&response.body_string().unwrap()).unwrap();
        assert!(action.is_none());

        let response = client.post("/play").dispatch();
        assert_eq!(response.status(), Status::Ok);

        let mut response = client.get("/front").dispatch();
        let action =
            serde_json::from_str::<Option<Action>>(&response.body_string().unwrap()).unwrap();
        assert_eq!(action.unwrap(), Action::Play);
    }

    #[test]
    fn test_stream() {
        let client = setup_rocket_test_client();
        let response = client.post("/pause").dispatch();
        assert_eq!(response.status(), Status::Ok);

        let response = client
            .post("/stream")
            .body(r#"{"url":"https://www.youtube.com/watch?v=9em32dDnTck"}"#)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        let mut response = client.get("/front").dispatch();
        let action =
            serde_json::from_str::<Option<Action>>(&response.body_string().unwrap()).unwrap();

        if let Some(Action::Stream(_)) = action {
            assert!(true)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn test_playback() {
        let client = setup_rocket_test_client();
        let mut response = client.get("/playback").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
        let state =
            serde_json::from_str::<PlaybackStatus>(&response.body_string().unwrap()).unwrap();
        assert!(!state.status);
    }

    #[test]
    fn test_inc() {
        let client = setup_rocket_test_client();
        let mut response = client.get("/volume").dispatch();
        let mut base_volume = serde_json::from_str::<Volume>(&response.body_string().unwrap())
            .unwrap()
            .volume;
        for _ in 0..10 {
            let response = client.post("/inc").dispatch();
            assert_eq!(response.status(), Status::Ok);
            let mut response = client.get("/volume").dispatch();
            let new_volume = serde_json::from_str::<Volume>(&response.body_string().unwrap())
                .unwrap()
                .volume;
            assert!(new_volume == base_volume + 1);
            base_volume = new_volume;
        }
        assert!(base_volume == 10);
        let response = client.post("/inc").dispatch();
        assert_eq!(response.status(), Status::BadRequest);

    }

    #[test]
    fn test_dec() {
        let client = setup_rocket_test_client();
        let mut response = client.get("/volume").dispatch();
        let base_volume = serde_json::from_str::<Volume>(&response.body_string().unwrap())
            .unwrap()
            .volume;
        assert!(base_volume == 0);
        let response = client.post("/dec").dispatch();
        assert_eq!(response.status(), Status::BadRequest);
        let response = client.post("/inc").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let response = client.post("/dec").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let mut response = client.get("/volume").dispatch();
        let new_volume = serde_json::from_str::<Volume>(&response.body_string().unwrap())
            .unwrap()
            .volume;
        assert!(new_volume == base_volume);
    }

    #[test]
    fn test_set_volume() {
        let client = setup_rocket_test_client();
        let response = client.post("/volume").body(r#"{"volume": 5}"#).dispatch();
        assert_eq!(response.status(), Status::Ok);
        let mut response = client.get("/volume").dispatch();
        let volume = serde_json::from_str::<Volume>(&response.body_string().unwrap())
            .unwrap()
            .volume;
        assert!(volume == 5);
        let response = client.post("/volume").body(r#"{"volume": 15}"#).dispatch();
        assert_eq!(response.status(), Status::BadRequest);
        let mut response = client.get("/volume").dispatch();
        let volume = serde_json::from_str::<Volume>(&response.body_string().unwrap())
            .unwrap()
            .volume;
        assert!(volume == 5);
        let response = client.post("/volume").body(r#"{"volume": -5}"#).dispatch();
        assert_eq!(response.status(), Status::BadRequest);
        let mut response = client.get("/volume").dispatch();
        let volume = serde_json::from_str::<Volume>(&response.body_string().unwrap())
            .unwrap()
            .volume;
        assert!(volume == 5)
    }

    #[test]
    fn test_seek() {
        let client = setup_rocket_test_client();
        let response = client.post("/seek").body(r#"{"ammount": 30}"#).dispatch();
        assert_eq!(response.status(), Status::Ok);
        let mut response = client.get("/front").dispatch();
        let action =
            serde_json::from_str::<Option<Action>>(&response.body_string().unwrap()).unwrap();
        if let Some(Action::Seek(amm)) = action {
            assert!(amm.ammount == 30);
        } else {
            assert!(false);
        }
        let client = setup_rocket_test_client();
        let response = client.post("/seek").body(r#"{"ammount": -600}"#).dispatch();
        assert_eq!(response.status(), Status::Ok);
        let mut response = client.get("/front").dispatch();
        let action =
            serde_json::from_str::<Option<Action>>(&response.body_string().unwrap()).unwrap();
        if let Some(Action::Seek(amm)) = action {
            assert!(amm.ammount == -600);
        } else {
            assert!(false);
        }
        let client = setup_rocket_test_client();
        // incorrect ammount
        let response = client.post("/seek").body(r#"{"ammount": -125}"#).dispatch();
        assert_eq!(response.status(), Status::BadRequest);
        let mut response = client.get("/front").dispatch();
        let action =
            serde_json::from_str::<Option<Action>>(&response.body_string().unwrap()).unwrap();
        assert!(action.is_none());
    }

    #[test]
    fn test_skip() {
        let client = setup_rocket_test_client();
        let response = client.post("/skip").dispatch();
        assert_eq!(response.status(), Status::Ok);

        let mut response = client.get("/front").dispatch();
        let action =
            serde_json::from_str::<Option<Action>>(&response.body_string().unwrap()).unwrap();
        assert_eq!(action.unwrap(), Action::Skip);
    }
}
