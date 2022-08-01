use std::collections::BTreeSet;

use async_trait::async_trait;
use futures::future;
use uuid::Uuid;

use crate::html::event::{Change, Event, Feedback};
use crate::html::{Handler, Html, Refresh, ToHtml};
use crate::share::{ShareChildren, ShareRead, ShareUpdate, ShareWrite};
use crate::task::edit::edit_shared;
use crate::task::edit::edit_shared::EditShared;
use crate::task::{TaskValue, Value};

#[derive(Clone, Debug)]
pub struct EditVec<S, T> {
    container_id: Uuid,
    elements_id: Uuid,
    add_id: Uuid,
    rows: Vec<Row>,
    share: S,
    tasks: Vec<T>,
}

impl<S, T> EditVec<S, T>
where
    S: ShareChildren,
    S::Child: ShareRead<Value = T::Output> + Clone,
    T: Value,
    T::Output: EditShared<S::Child, Task = T>,
{
    pub fn new(share: S) -> Self {
        let tasks: Vec<T> = share.children().iter().cloned().map(edit_shared).collect();
        let rows = tasks.iter().map(|_| Row::new()).collect();
        EditVec {
            container_id: Uuid::new_v4(),
            elements_id: Uuid::new_v4(),
            add_id: Uuid::new_v4(),
            rows,
            share,
            tasks,
        }
    }
}

#[async_trait]
impl<S, T> Value for EditVec<S, T>
where
    S: ShareRead + Send + Sync,
    S::Value: Clone,
    T: Send + Sync,
{
    type Output = S::Value;

    async fn value(&self) -> TaskValue<Self::Output> {
        self.share.read().as_ref().clone()
    }
}

#[async_trait]
impl<S, T> Handler for EditVec<S, T>
where
    S: ShareChildren + Send,
    S::Child: ShareWrite,
    T: Value + Handler + Send + Sync,
    T::Output: Clone,
{
    async fn on_event(&mut self, event: Event) -> Feedback {
        match event {
            Event::Press { id } if id == self.add_id => {
                // Add a new row
                self.share
                    .children()
                    .push(<S::Child as ShareWrite>::create(TaskValue::Empty));
                Feedback::new()
            }
            Event::Press { id } if self.rows.iter().any(|row| row.remove_id == id) => {
                // Remove an existing row
                let index = self
                    .rows
                    .iter()
                    .position(|row| row.remove_id == id)
                    .unwrap();
                self.share.children().remove(index);

                Feedback::new()
            }
            _ => future::join_all(
                self.tasks
                    .iter_mut()
                    .map(|task| task.on_event(event.clone())),
            )
            .await
            .into_iter()
            .collect(),
        }
    }
}

#[async_trait]
impl<S, T> Refresh for EditVec<S, T>
where
    S: ShareChildren + ShareUpdate + Send + Sync,
    S::Child: ShareRead + Clone,
    <S::Child as ShareRead>::Value: EditShared<S::Child, Task = T>,
    T: ToHtml + Send + Sync,
{
    async fn refresh(&mut self, ids: &BTreeSet<Uuid>) -> Feedback {
        if self.share.updated(&ids) {
            self.tasks = self
                .share
                .children()
                .iter()
                .cloned()
                .map(edit_shared)
                .collect();
            self.rows = self.tasks.iter().map(|_| Row::new()).collect();
            Feedback::from(Change::Replace {
                id: self.container_id,
                html: self.to_html().await,
            })
        } else {
            Feedback::new()
        }
    }
}

#[async_trait]
impl<S, T> ToHtml for EditVec<S, T>
where
    S: Send + Sync,
    T: ToHtml + Send + Sync,
{
    async fn to_html(&self) -> Html {
        let children: Html = future::join_all(self.tasks.iter().map(ToHtml::to_html))
            .await
            .into_iter()
            .zip(&self.rows)
            .map(|(task, row)| {
                Html(format!(
                    r#"
                        <div id={}>
                            {task}
                            <button id="{}" class="button" type="button" onclick="press(this)">-</button>
                        </div>
                    "#,
                    row.container_id, row.remove_id
                ))
            })
            .collect();

        Html(format!(
            r#"
                <div id="{}" class="column">
                    <div id="{}" class="column">{children}</div>
                    <button id="{}" class="button" type="button" onclick="press(this)">+</button>
                </div>
            "#,
            self.container_id, self.elements_id, self.add_id
        ))
    }
}

#[derive(Clone, Debug)]
struct Row {
    container_id: Uuid,
    remove_id: Uuid,
}

impl Row {
    fn new() -> Self {
        Row {
            container_id: Uuid::new_v4(),
            remove_id: Uuid::new_v4(),
        }
    }
}
