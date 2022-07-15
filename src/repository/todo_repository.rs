use crate::diesel::prelude::*;
use crate::models::todo::{Todo, TodoUpdateRequest};
use crate::repository::db_context;
use crate::schema::todos::dsl::*;

#[derive(Default)]
pub struct TodoRepository {}

impl TodoRepository {
    pub fn get_all(&self) -> Vec<Todo> {
        let connection = db_context::establish_connection();
        todos
            .load::<Todo>(&connection)
            .expect("Error loading posts")
    }

    pub fn get_by_id(&self, todo_id: i32) -> Option<Todo> {
        let connection = db_context::establish_connection();
        todo!();
    }

    pub fn insert(&self, entity: Todo) -> Result<Todo, String> {
        todo!();
    }

    pub fn update(&self, todo_id: i32, entity: TodoUpdateRequest) -> Result<Todo, String> {
        todo!();
    }

    pub fn delete(&self, todo_id: i32) -> Result<bool, String> {
        todo!();
    }
}
