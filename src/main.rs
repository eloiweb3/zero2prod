// clippy and code quality config
#![deny(warnings)]
#![deny(
    clippy::all,
    clippy::unwrap_used,
    clippy::unnecessary_unwrap,
    clippy::pedantic
)]


use std::net::TcpListener;
use zero2prod::startup::run;
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
        let listener = TcpListener::bind("127.0.0.1:0")?;

  let _ = run(listener)?.await;
    Ok(())
}
