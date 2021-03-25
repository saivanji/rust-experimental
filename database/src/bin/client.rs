use anyhow::Result;
use clap::{App, Arg, SubCommand};

fn main() -> Result<()> {
    let version = env!("CARGO_PKG_VERSION");
    let address_arg = Arg::with_name("address")
        .long("addr")
        .help("Sets address of a server to operate on")
        .takes_value(true)
        .required(true);

    let matches = App::new("Key/Value database client")
        .version(version)
        .subcommand(
            SubCommand::with_name("get")
                .about("Get the string value of a given string key")
                .arg(Arg::with_name("key").help("A string key").required(true))
                .arg(&address_arg),
        )
        .subcommand(
            SubCommand::with_name("set")
                .about("Sets a given key to a given value")
                .arg(Arg::with_name("key").help("A string key").required(true))
                .arg(
                    Arg::with_name("value")
                        .help("A string value of a key")
                        .required(true),
                )
                .arg(&address_arg),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Removes a given key")
                .arg(Arg::with_name("key").help("A string key").required(true))
                .arg(&address_arg),
        )
        .get_matches();

    match matches.subcommand() {
        ("get", Some(matches)) => {
            let address = matches.value_of("address").unwrap();
            let key = matches.value_of("key").unwrap();
            println!("GET. {}, address - {}", key, address);
        }
        ("set", Some(matches)) => {
            let address = matches.value_of("address").unwrap();
            let key = matches.value_of("key").unwrap();
            println!("SET. {}, address - {}", key, address);
        }
        ("rm", Some(matches)) => {
            let address = matches.value_of("address").unwrap();
            let key = matches.value_of("key").unwrap();
            println!("RM. {}, address - {}", key, address);
        }
        _ => unreachable!(),
    }

    Ok(())
}
