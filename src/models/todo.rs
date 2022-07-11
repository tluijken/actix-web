use super::*;

/// Task to do.
#[derive(Serialize, Deserialize, Component, Clone, Debug)]
pub struct Todo {
    /// Unique id for the todo item.
    #[component(example = 1)]
    pub id: i32,
    /// Description of the taks to do.
    #[component(example = "Remember to buy groceries")]
    pub value: String,
    /// Mark is the task done or not
    pub checked: bool,
}

/// Request to update existing `Todo` item.
#[derive(Serialize, Deserialize, Component, Clone, Debug)]
pub struct TodoUpdateRequest {
    /// Optional new value for the `Todo` task.
    #[component(example = "Dentist at 14.00")]
    pub value: Option<String>,
    /// Optional check status to mark is the task done or not.
    pub checked: Option<bool>,
}

/// Search todos Query
#[derive(Deserialize, Debug, IntoParams)]
pub struct SearchTodos {
    /// Content that should be found from Todo's value field
    pub value: String,
}
