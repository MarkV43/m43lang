use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, web::Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub code: String,
}

impl Request {
    pub fn save_to_file() {
        let mut file = 
    }
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/wasm")]
async fn wasm(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}   

#[post("/echo")]
async fn echo(req_body: Json<Request>) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(wasm)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8082))?
    .run()
    .await
}