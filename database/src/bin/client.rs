use anyhow::Result;
use clap::{App, Arg, SubCommand};
use database::Client;

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
            let mut client = Client::connect(address)?;

            match client.get(String::from(key))? {
                Some(value) => println!("{}", value),
                None => println!("No data found for {} key", key),
            }
        }
        ("set", Some(matches)) => {
            let address = matches.value_of("address").unwrap();
            let key = matches.value_of("key").unwrap();
            let value = matches.value_of("value").unwrap();
            let mut client = Client::connect(address)?;

            client.set(String::from(key), String::from(value))?;
        }
        ("rm", Some(matches)) => {
            let address = matches.value_of("address").unwrap();
            let key = matches.value_of("key").unwrap();
            let mut client = Client::connect(address)?;

            client.remove(String::from(key))?;
        }
        _ => println!("Specify an action you want to apply. Use --help for further details"),
    }

    Ok(())
}
