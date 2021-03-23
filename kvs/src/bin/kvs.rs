use anyhow::Result;
use clap::{App, Arg, SubCommand};
use kvs::KvStore;
use std::env::current_dir;
use std::process;

fn main() -> Result<()> {
    let version = env!("CARGO_PKG_VERSION");

    let matches = App::new("Key/Value store")
        .version(version)
        .subcommand(
            SubCommand::with_name("get")
                .about("Get the string value of a given string key")
                .arg(Arg::with_name("KEY").help("A string key").required(true)),
        )
        .subcommand(
            SubCommand::with_name("set")
                .about("Sets a given key to a given value")
                .arg(Arg::with_name("KEY").help("A string key").required(true))
                .arg(
                    Arg::with_name("VALUE")
                        .help("A string value of a key")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Removes a given key")
                .arg(Arg::with_name("KEY").help("A string key").required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        ("get", Some(matches)) => {
            let store = KvStore::open(current_dir()?)?;
            let key = matches.value_of("KEY").unwrap();
            let value = store.get(key.to_owned())?;

            match value {
                Some(value) => {
                    println!("{}", value);
                }
                None => {
                    println!("Key not found");
                    process::exit(1);
                }
            }
        }
        ("set", Some(matches)) => {
            let key = matches.value_of("KEY").unwrap();
            let value = matches.value_of("VALUE").unwrap();
            let mut store = KvStore::open(current_dir()?)?;

            store.set(key.to_owned(), value.to_owned())?;
        }
        ("rm", Some(matches)) => {
            let key = matches.value_of("KEY").unwrap();
            let mut store = KvStore::open(current_dir()?)?;

            store.remove(key.to_owned())?;
        }
        _ => unreachable!(),
    }

    Ok(())
}
