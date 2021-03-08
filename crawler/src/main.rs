mod addr;
mod html;
mod page;
mod storage;
mod trail;
mod utils;

use anyhow::Result;
use async_std::task;
use clap::{App, Arg};
use std::process;

use addr::entrypoint::Entrypoint;
use addr::link::Link;
use html::markup::Markup;
use html::node::{Node, NodeKind};
use storage::file::File;
use storage::location::Location;
use trail::Trail;

fn main() {
    match task::block_on(start()) {
        Ok(_) => println!("Crawled successfully"),
        Err(msg) => {
            println!("{}", msg);
            process::exit(1)
        }
    }
}

async fn start() -> Result<()> {
    let matches = App::new("Web crawler")
        .version("1.0.0")
        .about("Recursively downloads specified website")
        .arg(
            Arg::with_name("website")
                .short("w")
                .long("website")
                .required(true)
                .help("Website to crawl"),
        )
        .arg(
            Arg::with_name("path")
                .short("p")
                .long("path")
                .required(true)
                .help("Path for saving results to. Relative to './out' dir"),
        )
        .get_matches();

    let website = matches.value_of("website").unwrap();
    let path = matches.value_of("path").unwrap();

    let mut trail = Trail::new();
    let workdir = Location::new(path);
    let entrypoint = Entrypoint::parse(&website)?;

    storage::cleanup(&workdir)?;
    page::process("/", &entrypoint, &workdir, &mut trail).await?;

    Ok(())
}
