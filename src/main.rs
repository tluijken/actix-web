mod api;
use actix_web::{App, HttpServer};
use paperclip::actix::{
    // extension trait for actix_web::App and proc-macro attributes
    OpenApiExt, 
    // If you prefer the macro syntax for defining routes, import the paperclip macros
    // get, post, put, delete
    // use this instead of actix_web::web
    web::{self},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        // Record services and routes from this line.
        .wrap_api()
        // Add routes like you normally do...
        .service(
            web::resource("/blogs").route(web::get().to(api::blogcontroller::get)),
        )
        .service(
            web::resource("/blogs/{id}").route(web::get().to(api::blogcontroller::get_single)),
        )
        .with_json_spec_v3_at("/api/spec/v3")
        .build()
    ).bind("127.0.0.1:8080")?
    .run().await
}
