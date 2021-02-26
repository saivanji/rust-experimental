mod input;
mod page;

use anyhow::Result;
use async_std::task;
use std::env;
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

fn create_dirs(path: &Path) {
    println!("{:?}", path);
}

async fn start() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let website = input::match_website(&args)?;
    let path = input::match_path(&args)?;

    create_dirs(path);
    page::process_page(&website).await?;

    Ok(())
}
