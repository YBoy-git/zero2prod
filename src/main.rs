use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr()?.to_string();
    println!("Starting server on {address}");
    run(listener)?.await
}
