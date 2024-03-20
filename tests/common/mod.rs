use std::net::TcpListener;

use sqlx::{migrate, query, Connection, PgConnection, PgPool};
use uuid::Uuid;
use zero2prod::configuration::{
    get_configuration, AppConnection, Configuration, DatabaseConfiguration,
};
use zero2prod::startup::run;

use sqlx::Executor;

pub struct TestApp {
    pub config: Configuration,
    pub pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    let mut config = Configuration {
        connection: AppConnection {
            port: 0,
            ..Default::default()
        },
        ..get_configuration().unwrap()
    };
    config.database.name = Uuid::new_v4().to_string();

    let listener = TcpListener::bind(config.connection.to_string()).unwrap();
    config.connection.port = listener.local_addr().unwrap().port();

    let pool = configure_database(&config.database).await;

    let server = run(listener, pool.clone()).unwrap();
    tokio::spawn(server);

    TestApp { config, pool }
}

async fn configure_database(config: &DatabaseConfiguration) -> PgPool {
    let mut connection = PgConnection::connect(&config.instance_url()).await.unwrap();
    connection
        .execute(query(&format!(r#"CREATE DATABASE "{}";"#, config.name)))
        .await
        .unwrap();

    let pool = PgPool::connect(&config.database_url()).await.unwrap();
    migrate!("./migrations").run(&pool).await.unwrap();

    pool
}
