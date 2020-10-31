#![feature(proc_macro_hygiene, decl_macro)]
use lib::Config as CliViewConfig;
use lib::GenericResult as Result;
use lib::QueueState;
use lib::Url;
use lib::{extract_url, QueueStateSendable};
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

fn setup_rocket(cfg: CliViewConfig) -> rocket::Rocket {
    let rocket_config = Config::build(Environment::Development)
        .address("0.0.0.0")
        .port(cfg.queue_port)
        .workers(cfg.num_workers)
        .finalize()
        .unwrap();

    rocket::custom(rocket_config)
        .mount("/", rocket::routes![front, queue_get, queue_post])
        .manage(QueueState::new())
}
fn main() -> Result<()> {
    let cfg = CliViewConfig::load()?;
    let rocket = setup_rocket(cfg);
    rocket.launch();

    Ok(())
}

#[cfg(test)]
mod test {
    use super::setup_rocket;
    use lib::Config as CliViewConfig;
    use lib::QueueStateSendable;
    use lib::Url;
    use rocket::http::ContentType;
    use rocket::http::Status;
    use rocket::local::Client;

    fn setup_rocket_test_client() -> Client {
        let cfg = CliViewConfig::load().unwrap();
        Client::new(setup_rocket(cfg)).expect("valid rocket instance")
    }
    #[test]
    fn test_empty_queue() {
        let client = setup_rocket_test_client();
        let mut response = client.get("/queue").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
        let state =
            serde_json::from_str::<QueueStateSendable>(&response.body_string().unwrap()).unwrap();
        assert!(state.queue.is_empty());
    }

    #[test]
    fn test_add_to_queue() {
        let client = setup_rocket_test_client();
        let response = client
            .post("/queue")
            .body(r#"{"url":"https://www.youtube.com/watch?v=9em32dDnTck"}"#)
            .header(ContentType::JSON)
            .dispatch();
        assert_eq!(response.status(), Status::Accepted);
    }

    #[test]
    fn test_add_to_queue_invalid_url() {
        let client = setup_rocket_test_client();
        let response = client
            .post("/queue")
            .body(r#"{"url":"https://www.badurl.gg"}"#)
            .header(ContentType::JSON)
            .dispatch();
        assert_eq!(response.status(), Status::BadRequest);
    }

    #[test]
    fn test_front() {
        let client = setup_rocket_test_client();
        let response = client
            .post("/queue")
            .body(r#"{"url":"https://www.youtube.com/watch?v=9em32dDnTck"}"#)
            .header(ContentType::JSON)
            .dispatch();
        assert_eq!(response.status(), Status::Accepted);

        let mut response = client.get("/front").dispatch();
        let state = serde_json::from_str::<Option<Url>>(&response.body_string().unwrap()).unwrap();
        assert!(state.is_some());

        let mut response = client.get("/front").dispatch();
        let state = serde_json::from_str::<Option<Url>>(&response.body_string().unwrap()).unwrap();
        assert!(state.is_none());
    }
}
