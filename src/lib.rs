use actix_web::{dev::Server, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

async fn index() -> impl Responder {
    HttpResponse::Ok().body(include_str!("../index.html"))
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello, {}!", name)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    Ok(HttpServer::new(|| {
            App::new()
                .route("/", web::get().to(index))
                .route("/health_check", web::get().to(health_check))
                .route("/{name}", web::get().to(greet))
        })
        .listen(listener)?
        .run())
}
