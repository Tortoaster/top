use std::ops::Not;

use log::info;

use toprs::integration::axum::TopRsRouter;
pub use toprs::prelude::*;
use toprs::task::combinator::{Action, Continuation};
use toprs::task::value::TaskValue;

fn name() -> impl Task {
    enter::<String>().step(vec![
        Continuation::OnValue(Box::new(|value: TaskValue<String>| {
            value
                .into_option()
                .and_then(|name| (&name == "Rick").then(|| update(format!("Hi, {name}!"))))
        })),
        Continuation::OnAction(
            Action::new("Hi"),
            Box::new(|value: TaskValue<String>| {
                value.into_option().and_then(|name| {
                    name.is_empty()
                        .not()
                        .then(|| update(format!("Hello, {name}!")))
                })
            }),
        ),
    ])
}

const HOST: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Listening on http://{HOST}");

    let router = TopRsRouter::new().task("/", name);

    axum::Server::bind(&HOST.parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
