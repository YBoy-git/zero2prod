use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");

    let listener =
        TcpListener::bind(format!("127.0.0.1:{}", configuration.connection.port)).unwrap();
    let address = listener.local_addr()?.to_string();
    let pool = PgPool::connect(&configuration.database.database_url())
        .await
        .expect("Failed to connect to the database.");

    println!("Starting server on {address}");

    run(listener, pool)?.await
}
