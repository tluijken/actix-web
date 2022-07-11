use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path, Query, ServiceConfig},
    HttpResponse, Responder,
};
use utoipa::IntoParams;

use crate::middleware::{log_api_key::LogApiKey, require_api_key::RequireApiKey};

use crate::models::{
    error_response::ErrorResponse,
    todo::{SearchTodos, Todo, TodoUpdateRequest},
};
use crate::repository::todo_repository::TodoRepository;

pub mod todo_controller;
