use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;

use crate::routes::{health_check, index, subscribe};

pub fn run(listener: TcpListener, pool: PgPool) -> std::io::Result<Server> {
    let pool = web::Data::new(pool);

    Ok(HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(index))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(pool.clone())
    })
    .listen(listener)?
    .run())
}
