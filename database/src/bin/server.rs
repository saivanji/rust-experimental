use anyhow::Result;
use clap::{App, Arg};
use database::{DefaultEngine, Server};
use std::env::current_dir;
use std::net::SocketAddr;

fn main() -> Result<()> {
    let version = env!("CARGO_PKG_VERSION");

    let matches = App::new("Key/Value database server")
        .version(version)
        .arg(
            Arg::with_name("address")
                .long("addr")
                .help("Sets address of a server to operate on")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("engine")
                .long("engine")
                .help("Sets engine for a server to use")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let address = matches.value_of("address").unwrap().parse::<SocketAddr>()?;
    // let engine = matches.value_of("engine").unwrap();

    let engine = DefaultEngine::open(current_dir()?)?;
    Server::new(engine).run_with_callback(address, || {
        println!("Database server is listening at {}", address)
    })?;

    Ok(())
}
