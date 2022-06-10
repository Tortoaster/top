//! This module contains functionality for the interaction between the user and the server.

use crate::html::{Handler, ToHtml};
use crate::task::Task;

pub mod choice;
pub mod container;
pub mod convert;
pub mod generic;
pub mod primitive;
pub mod tuple;

/// Editors describe how tasks should respond to user input, and how data can be retrieved from it.
pub trait Editor: Task + Handler + ToHtml {}
