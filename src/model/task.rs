use serde::{Deserialize, Serialize};

use super::base::BaseModelControler;

pub struct Task {
    id: i64,
    title: String,
    author: i64,
}

#[derive(Deserialize)]
pub struct TaskForCreate {
    title: String,
}

#[derive(Deserialize)]
pub struct TaskForUpdate {
    title: String,
}

pub struct TaskModelControler;

impl BaseModelControler for TaskModelControler {
    const TABLE: &'static str = "task";
}
