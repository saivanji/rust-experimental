mod addr;
mod html;
mod page;
mod storage;
mod trail;
mod utils;

use anyhow::{anyhow, Result};
use async_std::task;
use std::env;
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
    let args: Vec<String> = env::args().collect();
    let website = match_website(&args)?;
    let path = match_path(&args)?;

    let trail = Trail::new();
    let workdir = Location::new(path);
    let entrypoint = Entrypoint::parse(&website)?;

    storage::cleanup(&workdir)?;
    page::process("/", &entrypoint, &workdir, &trail).await?;

    Ok(())
}

pub fn match_website(args: &Vec<String>) -> Result<&str> {
    match args.get(1) {
        Some(url) => Ok(url),
        None => Err(anyhow!("Please specify website url to crawl")),
    }
}

pub fn match_path(args: &Vec<String>) -> Result<&str> {
    match args.get(2) {
        Some(dir) => Ok(dir),
        None => Err(anyhow!("Please specify directory")),
    }
}
