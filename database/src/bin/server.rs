use anyhow::Result;
use clap::{App, Arg};

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

    let address = matches.value_of("address").unwrap();
    let engine = matches.value_of("engine").unwrap();

    println!("address - {}, engine - {}", address, engine);

    Ok(())
}
