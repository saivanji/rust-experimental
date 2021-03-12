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
use std::time::Instant;

use addr::entrypoint::Entrypoint;
use addr::link::Link;
use html::markup::Markup;
use html::node::{Node, NodeKind};
use storage::file::File;
use storage::location::Location;
use trail::Trail;

fn main() {
    let start_time = Instant::now();

    let matches = App::new("Web crawler")
        .version("1.0.0")
        .about("Recursively downloads specified website")
        .arg(
            Arg::with_name("website")
                .required(true)
                .index(1)
                .help("Website to crawl"),
        )
        .arg(
            Arg::with_name("dest")
                .required(true)
                .index(2)
                .help("Destination path for saving results to. Relative to './out' dir"),
        )
        .get_matches();

    let website = matches.value_of("website").unwrap();
    let dest = matches.value_of("dest").unwrap();

    match crawl(website, dest) {
        Ok(_) => {
            let duration = start_time.elapsed().as_secs();

            println!("Crawled successfully. It took {} seconds", duration);
        }
        Err(msg) => {
            println!("{}", msg);
            process::exit(1)
        }
    }
}

fn crawl(website: &str, dest: &str) -> Result<()> {
    let mut trail = Trail::new();
    let workdir = Location::new(dest);
    let entrypoint = Entrypoint::parse(website)?;

    storage::cleanup(&workdir)?;

    task::block_on(page::process("/", &entrypoint, &workdir, &mut trail))?;

    Ok(())
}
