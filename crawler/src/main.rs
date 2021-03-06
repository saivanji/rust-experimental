#![feature(str_split_once)]

mod file;
mod link;
mod location;
mod markup;
mod node;

use anyhow::{anyhow, Result};
use async_std::task;
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::process;

use file::File;
use location::Location;
use markup::Markup;
use node::{Node, NodeKind};

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
        fs::remove_dir_all(workdir.path).or(Err(anyhow!("Can not remove website directory")))?;
    }

    Ok(())
}

async fn start() -> Result<()> {
    let mut trail: BTreeSet<String> = BTreeSet::new();

    let args: Vec<String> = env::args().collect();

    let website = match_website(&args)?;
    let path = match_path(&args)?;
    let workdir = Location::new(path);

    cleanup(&workdir)?;
    // page::process_page(&website, &workdir, &mut trail).await?;

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
