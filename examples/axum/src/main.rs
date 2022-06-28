use std::env;
use std::net::SocketAddr;

use axum::response::Html;
use axum::routing::get;
use axum::{Router, Server};
use log::debug;

use top::integration::axum::{task, Task, TopService};
use top::prelude::*;
use top::task::view::option::ViewOption;
use top::task::Value;

async fn index() -> Html<&'static str> {
    Html(
        r#"
<a href="/one">1</a><br/>
<a href="/two">2</a><br/>
<a href="/three">3</a><br/>
<a href="/four">4</a><br/>
<a href="/five">5</a><br/>
<a href="/six">6</a><br/>
<a href="/seven">7</a><br/>
<a href="/eight">8</a><br/>
<a href="/nine">9</a><br/>
<a href="/ten">10</a><br/>
"#,
    )
}

// Inspect
async fn one() -> impl Task {
    view("Hello, world!")
}

// Interact
async fn two() -> impl Task {
    edit("Hello, world!".to_string())
}

// Sequential
async fn three() -> impl Task {
    enter::<u32>().then(
        Trigger::Button(Button::new("Press me!")),
        |value| value.as_ref().map(|n| *n == 20).unwrap_or_default(),
        |value| view(value.unwrap() + 1),
    )
}

async fn four() -> impl Task {
    enter::<u32>().then(
        Trigger::Update,
        |value| value.as_ref().map(|n| *n == 20).unwrap_or_default(),
        |value| view(value.unwrap() + 1),
    )
}

// Parallel
async fn five() -> impl Task {
    view("B").and(view(5i32))
}

async fn six() -> impl Task {
    enter::<i32>().or(view(5))
}

async fn seven() -> impl Task {
    view("B").left(view(5)).then(
        Trigger::Button(Button::new("Ok")),
        |_| true,
        |value| view(value.unwrap()),
    )
}

async fn eight() -> impl Task {
    view("B").right(view(5)).then(
        Trigger::Button(Button::new("Ok")),
        |_| true,
        |value| view(value.unwrap()),
    )
}

// Shares
async fn nine() -> impl Task {
    let share: Shared<String> = Shared::new(TaskValue::Empty);

    edit_share(share.clone()).and(view_shared(share))
}

async fn ten() -> impl Task {
    let share: Shared<String> = Shared::new(TaskValue::Empty);
    let uwuified = share.map(|s| s.as_ref().map(|s| uwuifier::uwuify_str_sse(s)));

    edit_share(share).right(view_shared(uwuified)).then(
        Trigger::Update,
        |value| value.as_ref().map(|s| s.contains("x3")).unwrap_or_default(),
        |value| view(value.unwrap()).with_color(Color::Pink),
    )
}

// Advanced editors
// ...

// Derive macro
// ...

async fn test() -> impl Task {
    let task = edit(Some(5));
    let share = task.share().await;
    let other = ViewOption::new_shared(share);

    task.and(other)
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let router = Router::new()
        .nest("/top", TopService::new())
        .route("/", get(index))
        .route("/one", task(one))
        .route("/two", task(two))
        .route("/three", task(three))
        .route("/four", task(four))
        .route("/five", task(five))
        .route("/six", task(six))
        .route("/seven", task(seven))
        .route("/eight", task(eight))
        .route("/nine", task(nine))
        .route("/ten", task(ten))
        .route("/test", task(test));

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
