use std::env;
use std::net::SocketAddr;

use axum::{Router, Server};
use log::debug;

use top::integration::axum::{task, Task, TopService};
use top::share::{ShareValue, ShareVec};
use top::task::edit::{edit, edit_shared, enter, EditValue};
use top::task::parallel::TaskParallelExt;
use top::task::sequential::{always, has_value, Button, TaskSequentialExt, Trigger};
use top::task::view::{view, view_shared};

fn index() -> impl Task {
    enter::<i32>()
        .step()
        .on(Trigger::Button(Button::new("Ok")), has_value, |value| {
            view(value.unwrap() + 1)
        })
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let router = Router::new()
        .nest("/top", TopService::new())
        .route("/", task(index));

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
