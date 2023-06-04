use actix_web::dev::Server;
use actix_web::web::Form;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::net::TcpListener;

#[derive(Deserialize)]
struct FormData {
    name: String,
    email: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/subscriptions")]
async fn subscribe(Form(_form): Form<FormData>) -> impl Responder {
    HttpResponse::Ok()
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(health_check)
            .service(subscribe)
            .route("/manual_hello", web::get().to(manual_hello))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
