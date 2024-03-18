use std::net::TcpListener;

#[cfg(test)]
pub fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();

    let address = listener.local_addr().unwrap().to_string();

    let server = zero2prod::run(listener).unwrap();
    tokio::spawn(server);

    format!("http://{address}")
}
