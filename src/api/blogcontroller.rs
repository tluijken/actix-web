use std::fmt::{self};
use paperclip::actix::api_v2_errors;
use actix_web::{Error, 
    HttpResponse, 
    ResponseError, 
    http::StatusCode
};

use paperclip::actix::{
    // extension trait for actix_web::App and proc-macro attributes
    Apiv2Schema, api_v2_operation,
    // If you prefer the macro syntax for defining routes, import the paperclip macros
    // get, post, put, delete
    // use this instead of actix_web::web
    web::{Json, Path},
};
use serde::{Serialize, Deserialize};

#[api_v2_errors(
    code=400,
    code=401, description="Unauthorized: Can't read session from header",
    code=500,
)]
#[derive(Debug)]
pub enum HttpStatusErrors {
    BadRequest,
    Unauthorized,
    NotFound,
    InternalServerError,
}

impl fmt::Display for HttpStatusErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for HttpStatusErrors {
 fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            HttpStatusErrors::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            HttpStatusErrors::BadRequest => StatusCode::BAD_REQUEST,
            HttpStatusErrors::NotFound => StatusCode::NOT_FOUND,
            HttpStatusErrors::Unauthorized => StatusCode::UNAUTHORIZED
        }
    }
}

#[derive(Serialize, Deserialize, Apiv2Schema, Clone)]
pub struct Blog {
    name: String,
    body: String,
    id: i64,
}

fn load_blogs() -> Vec<Blog> {
    let result: Vec<Blog> = vec![
        Blog {
            name: "getting started with rust".to_string(),
            body: "lets get rusty".to_string(),
            id: 1
        },
        Blog {
            name: "paperclip is pretty cool".to_string(),
            body: "It allows me to do lots of things".to_string(),
            id: 2
        }
    ];
    result
}

#[api_v2_operation]
pub async fn get() -> Result<Json<Vec<Blog>>, Error> {
    Ok(Json(load_blogs()))
}

#[api_v2_operation]
pub async fn get_single(id: Path<i64>) -> Result<Json<Blog>, HttpStatusErrors> {
    let id_val: i64 = id.into_inner();

    if id_val <= 0 {
        println!("The given id has a value of {} but should be greater than 0", id_val);
        return Err(HttpStatusErrors::BadRequest);   
    }

    println!("getting blog with id {}", id_val);
    let blog = load_blogs().into_iter().find(|blog| blog.id == id_val);

    match blog {
        Some(blog_item) => Ok(Json(blog_item)),
        None => {
            println!("Didn't find blog with id {}", id_val);
            Err(HttpStatusErrors::NotFound)
        },
    }
}
