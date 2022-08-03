use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;

use axum::{Router, Server};
use lazy_static::lazy_static;
use log::debug;

use top::integration::axum::{task, TopService};
use top::share::{ShareRead, ShareValue, ShareVec, ShareWrite};
use top::task::edit::{edit, edit_shared, enter, EditValue, EditVec};
use top::task::parallel::{Parallel, TaskParallelExt};
use top::task::sequential::{always, has_value, Button, Sequential, TaskSequentialExt, Trigger};
use top::task::view::{view, view_shared, ViewDisplay, ViewVec};
use top::task::{Task, TaskValue};

type IndexTask = Sequential<
    Parallel<
        ViewVec<ShareVec<ShareValue<String>>, ViewDisplay<ShareValue<String>>>,
        EditValue<ShareValue<String>>,
        fn(TaskValue<Vec<String>>, TaskValue<String>) -> TaskValue<String>,
    >,
    Infallible,
>;

fn index() -> IndexTask {
    view_shared(MESSAGES.clone())
        .right(enter::<String>())
        .step()
        .on(Trigger::Button(Button::new("Send")), has_value, |message| {
            let mut current = MESSAGES.read().as_ref().clone().unwrap();
            current.push(message.unwrap());
            MESSAGES.write(TaskValue::Unstable(current));
            index()
        })
}

lazy_static! {
    static ref MESSAGES: ShareVec<ShareValue<String>> = ShareVec::new(Some(Vec::new()));
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
