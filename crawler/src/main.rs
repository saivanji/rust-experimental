mod entrypoint;
mod file;
mod link;
mod location;
mod markup;
mod node;
mod trail;
mod utils;

use anyhow::{anyhow, Result};
use async_std::task;
use std::env;
use std::fs;
use std::process;

use entrypoint::Entrypoint;
use file::File;
use link::Link;
use location::Location;
use markup::Markup;
use node::{Node, NodeKind};
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

fn cleanup(workdir: &Location) -> Result<()> {
    if workdir.path.exists() {
        fs::remove_dir_all(&workdir.path).or(Err(anyhow!("Can not remove website directory")))?;
    }

    Ok(())
}

async fn start() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let website = match_website(&args)?;
    let path = match_path(&args)?;

    let trail = Trail::new();
    let workdir = Location::new(path);
    let entrypoint = Entrypoint::parse(&website)?;

    cleanup(&workdir)?;

    // Should it be some sort of struct which also re-used in markup?
    let path = "/";
    let location = workdir.concat(path);

    let bytes = entrypoint.link(path).fetch().await?;
    let file = File::from(bytes, &location);

    file.persist();

    match Markup::parse(&file) {
        Some(markup) => markup.traverse(&trail, &entrypoint).await?,
        _ => (),
    }
    //

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
