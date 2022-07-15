use super::*;

/// Get list of todos.
///
/// List todos from in-memory todo store.
///
/// One could call the api endpoit with following curl.
/// ```text
/// curl localhost:8080/todo
/// ```
#[utoipa::path(
    responses(
        (status = 200, description = "List current todo items", body = [Todo])
    )
)]
#[get("/todo")]
pub async fn get_todos(todo_store: Data<TodoRepository>) -> impl Responder {
    let todos = todo_store.get_all();
    HttpResponse::Ok().json(todos.clone())
}

/// Create new Todo to shared in-memory storage.
///
/// Post a new `Todo` in request body as json to store it. Api will return
/// created `Todo` on success or `ErrorResponse::Conflict` if todo with same id already exists.
///
/// One could call the api with.
/// ```text
/// curl localhost:8080/todo -d '{"id": 1, "value": "Buy movie ticket", "checked": false}'
/// ```
#[utoipa::path(
    request_body = Todo,
    responses(
        (status = 201, description = "Todo created successfully", body = Todo),
        (status = 409, description = "Todo with id already exists", body = ErrorResponse, example = json!(ErrorResponse::Conflict(String::from("id = 1"))))
    )
)]
#[post("/todo")]
pub(super) async fn create_todo(
    todo: Json<Todo>,
    todo_store: Data<TodoRepository>,
) -> impl Responder {
    match todo_store.get_by_id(todo.id) {
        Some(existing) => {
            HttpResponse::Conflict().json(ErrorResponse::Conflict(format!("id = {}", existing.id)))
        }
        _ => HttpResponse::Ok().json(todo_store.insert(todo.clone())),
    }
}

/// Delete Todo by given path variable id.
///
/// This ednpoint needs `api_key` authentication in order to call. Api key can be found from README.md.
///
/// Api will delete todo from shared in-memory storage by the provided id and return success 200.
/// If storage does not contain `Todo` with given id 404 not found will be returned.
#[utoipa::path(
    responses(
        (status = 200, description = "Todo deleted successfully"),
        (status = 401, description = "Unauthorized to delete Todo", body = ErrorResponse, example = json!(ErrorResponse::Unauthorized(String::from("missing api key")))),
        (status = 404, description = "Todo not found by id", body = ErrorResponse, example = json!(ErrorResponse::NotFound(String::from("id = 1"))))
    ),
    params(
        ("id", description = "Unique storage id of Todo")
    ),
    security(
        ("api_key" = [])
    )
)]
#[delete("/todo/{id}", wrap = "RequireApiKey")]
pub(super) async fn delete_todo(id: Path<i32>, todo_store: Data<TodoRepository>) -> impl Responder {
    let id = id.into_inner();

    match todo_store.get_by_id(id) {
        Some(existing) => {
            todo_store.delete(id);
            HttpResponse::Ok().finish()
        }
        _ => HttpResponse::NotFound().json(ErrorResponse::NotFound(format!("id = {id}"))),
    }
}

/// Get Todo by given todo id.
///
/// Return found `Todo` with status 200 or 404 not found if `Todo` is not found from shared in-memory storage.
#[utoipa::path(
    responses(
        (status = 200, description = "Todo found from storage", body = Todo),
        (status = 404, description = "Todo not found by id", body = ErrorResponse, example = json!(ErrorResponse::NotFound(String::from("id = 1"))))
    ),
    params(
        ("id", description = "Unique storage id of Todo")
    )
)]
#[get("/todo/{id}")]
pub(super) async fn get_todo_by_id(
    id: Path<i32>,
    todo_store: Data<TodoRepository>,
) -> impl Responder {
    let id = id.into_inner();
    match todo_store.get_by_id(id) {
        Some(existing) => HttpResponse::Ok().json(existing),
        _ => HttpResponse::NotFound().json(ErrorResponse::NotFound(format!("id = {id}"))),
    }
}

/// Update Todo with given id.
///
/// This endpoint supports optional authentication.
///
/// Tries to update `Todo` by given id as path variable. If todo is found by id values are
/// updated according `TodoUpdateRequest` and updated `Todo` is returned with status 200.
/// If todo is not found then 404 not found is returned.
#[utoipa::path(
    request_body = TodoUpdateRequest,
    responses(
        (status = 200, description = "Todo updated successfully", body = Todo),
        (status = 404, description = "Todo not found by id", body = ErrorResponse, example = json!(ErrorResponse::NotFound(String::from("id = 1"))))
    ),
    params(
        ("id", description = "Unique storage id of Todo")
    ),
    security(
        (),
        ("api_key" = [])
    )
)]
#[put("/todo/{id}", wrap = "LogApiKey")]
pub(super) async fn update_todo(
    id: Path<i32>,
    todo: Json<TodoUpdateRequest>,
    todo_store: Data<TodoRepository>,
) -> impl Responder {
    let id = id.into_inner();
    let todo = todo.into_inner();

    let update_result = todo_store.update(id, todo);
    match update_result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(error_message) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn configure(store: Data<TodoRepository>) -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config
            .app_data(store)
            .service(get_todos)
            .service(create_todo)
            .service(delete_todo)
            .service(get_todo_by_id)
            .service(update_todo);
    }
}
