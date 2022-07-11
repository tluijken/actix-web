use crate::models::todo::Todo;
use std::sync::Mutex;

#[derive(Default)]
pub struct TodoRepository {
    pub todos: Mutex<Vec<Todo>>,
}
