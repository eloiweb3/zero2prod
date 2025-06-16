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
use zero2prod::configuration::get_configuration;
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    
    
    
    let listener = TcpListener::bind(address)?;

  let _ = run(listener)?.await;
    Ok(())
}
