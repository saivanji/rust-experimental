use anyhow::Result;
use bcrypt::{hash, DEFAULT_COST};
use clap::{App, Arg, SubCommand};
use std::env;
use store::Store;

#[tokio::main]
async fn main() -> Result<()> {
    let version = env!("CARGO_PKG_VERSION");

    let matches = App::new("Manager")
        .version(version)
        .subcommand(
            SubCommand::with_name("create_user")
                .about("Create a new user")
                .arg(Arg::with_name("USERNAME").help("User name").required(true))
                .arg(
                    Arg::with_name("PASSWORD")
                        .help("User password")
                        .required(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("create_user", Some(matches)) => {
            let username = matches.value_of("USERNAME").unwrap();
            let password = matches.value_of("PASSWORD").unwrap();
            let hash = hash(password, DEFAULT_COST)?;

            let addr = env::var("DATABASE_URL")?;
            let store = Store::new(addr).await?;

            store.create_user(username, &hash).await?;

            println!("User {} was created successfully", username);
        }
        _ => unreachable!(),
    }

    Ok(())
}
