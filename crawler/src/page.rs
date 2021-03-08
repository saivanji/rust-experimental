use crate::{Entrypoint, File, Location, Markup, Trail};
use anyhow::Result;
use async_recursion::async_recursion;

#[async_recursion(?Send)]
pub async fn process(
    path: &str,
    entrypoint: &Entrypoint,
    workdir: &Location,
    trail: &mut Trail,
) -> Result<()> {
    let file_loc = workdir.concat(path);
    let bytes = entrypoint.link(path).fetch().await?;
    let file = File::from(bytes, &file_loc);

    file.persist()?;
    trail.set(path);

    match Markup::parse(&file) {
        Some(markup) => markup.traverse(entrypoint, workdir, trail).await?,
        _ => (),
    }

    Ok(())
}
