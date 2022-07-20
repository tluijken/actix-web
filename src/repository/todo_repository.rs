use actix_web::web::Data;

use crate::diesel::prelude::*;
use crate::models::todo::{Todo, TodoUpdateRequest};
use crate::repository::db_context;
use crate::schema::todos::dsl::*;

#[derive(Default)]
pub struct TodoRepository {}

impl TodoRepository {
    pub fn get_all(&self, db_pool: Data<db_context::PostgresPool>) -> Vec<Todo> {
        todos
            .load::<Todo>(&db_pool.get().unwrap())
            .expect("Error loading posts")
    }

    pub fn get_by_id(&self, todo_id: i32) -> Option<Todo> {
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
