#![feature(str_split_once)]

mod input;
mod page;

use anyhow::{anyhow, Result};
use async_std::task;
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn main() {
    match task::block_on(start()) {
        Ok(_) => println!("Crawled successfully"),
        Err(msg) => {
            println!("{}", msg);
            process::exit(1)
        }
    }
}

fn cleanup(path: &Path) -> Result<()> {
    if path.exists() {
        fs::remove_dir_all(path).or(Err(anyhow!("Can not remove website directory")))?;
    }

    Ok(())
}

async fn start() -> Result<()> {
    let mut trail: BTreeSet<String> = BTreeSet::new();

    let args: Vec<String> = env::args().collect();

    let website = input::match_website(&args)?;
    let path = input::match_path(&args)?;
    let workdir = Path::new("./out").join(path);

    cleanup(&workdir)?;
    page::process_page(&website, &workdir, &mut trail).await?;

    Ok(())
}
