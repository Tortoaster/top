use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;

use axum::{Router, Server};
use lazy_static::lazy_static;
use log::debug;

use top::integration::axum::{task, TopService};
use top::share::{ShareRead, ShareValue, ShareVec, ShareWrite};
use top::task::edit::{enter, EditValue};
use top::task::parallel::{Parallel, TaskParallelExt};
use top::task::sequential::{has_value, Button, Sequential, TaskSequentialExt, Trigger};
use top::task::view::{view, view_shared, ViewDisplay, ViewVec};
use top::task::{Task, TaskValue};

type ChatTask = Sequential<
    Parallel<
        ViewVec<ShareVec<ShareValue<String>>, ViewDisplay<ShareValue<String>>>,
        EditValue<ShareValue<String>>,
        fn(TaskValue<Vec<String>>, TaskValue<String>) -> TaskValue<String>,
    >,
    Infallible,
>;

lazy_static! {
    static ref MESSAGES: ShareVec<ShareValue<String>> = ShareVec::new(Some(Vec::new()));
}

fn chat(name: String) -> ChatTask {
    view_shared(MESSAGES.clone())
        .right(enter::<String>())
        .step()
        .on(
            Trigger::Button(Button::new("Send")),
            has_value,
            move |message| {
                let mut messages_value = MESSAGES.read().as_ref().clone().unwrap();
                messages_value.push(format!("{}: {}", name, message.unwrap()));
                MESSAGES.write(TaskValue::Unstable(messages_value));
                chat(name)
            },
        )
}

fn index() -> impl Task {
    view("Please enter your name:")
        .right(enter::<String>())
        .step()
        .on(Trigger::Button(Button::new("Ok")), has_value, |name| {
            chat(name.unwrap())
        })
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let router = Router::new()
        .nest("/top", TopService::new())
        .route("/", task(|| index()));

    let host = env::var("HOST")
        .as_deref()
        .unwrap_or("127.0.0.1")
        .parse()
        .expect("invalid host");
    let port = env::var("PORT")
        .as_deref()
        .unwrap_or("8000")
        .parse()
        .expect("invalid port");
    let addr = SocketAddr::new(host, port);

    debug!("Listening on http://{addr}");
    Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
