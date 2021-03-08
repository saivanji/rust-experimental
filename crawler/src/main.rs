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

    match task::block_on(crawl(website, dest)) {
        Ok(_) => println!("Crawled successfully"),
        Err(msg) => {
            println!("{}", msg);
            process::exit(1)
        }
    }
}

async fn crawl(website: &str, dest: &str) -> Result<()> {
    let mut trail = Trail::new();
    let workdir = Location::new(dest);
    let entrypoint = Entrypoint::parse(website)?;

    storage::cleanup(&workdir)?;
    page::process("/", &entrypoint, &workdir, &mut trail).await?;

    Ok(())
}
