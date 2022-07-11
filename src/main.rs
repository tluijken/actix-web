use std::{error::Error, net::Ipv4Addr};

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;

mod models;

mod api;
mod middleware;
mod repository;

use repository::todo_repository::TodoRepository;

use models::{error_response::ErrorResponse, todo::Todo, todo::TodoUpdateRequest};

#[actix_web::main]
async fn main() -> Result<(), impl Error> {
    env_logger::init();

    #[derive(OpenApi)]
    #[openapi(
        handlers(
            api::todo_controller::get_todos,
            api::todo_controller::create_todo,
            api::todo_controller::delete_todo,
            api::todo_controller::get_todo_by_id,
            api::todo_controller::update_todo,
            api::todo_controller::search_todos
        ),
        components(Todo, TodoUpdateRequest, ErrorResponse),
        tags(
            (name = "todo", description = "Todo management endpoints.")
        ),
        modifiers(&SecurityAddon)
    )]
    struct ApiDoc;

    struct SecurityAddon;

    impl Modify for SecurityAddon {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            let components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("todo_apikey"))),
            )
        }
    }

    let store = Data::new(TodoRepository::default());
    // Make instance variable of ApiDoc so all worker threads gets the same instance.
    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        // This factory closure is called on each worker thread independently.
        App::new()
            .wrap(Logger::default())
            .configure(api::todo_controller::configure(store.clone()))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", openapi.clone()),
            )
    })
    .bind((Ipv4Addr::UNSPECIFIED, 8080))?
    .run()
    .await
}
