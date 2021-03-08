use crate::{Entrypoint, File, Location, Markup, Trail};
use anyhow::Result;

pub async fn process(
    path: &str,
    entrypoint: &Entrypoint,
    workdir: &Location,
    trail: &mut Trail,
) -> Result<()> {
    let location = workdir.concat(path);

    let bytes = entrypoint.link(path).fetch().await?;
    let file = File::from(bytes, &location);

    file.persist()?;
    trail.set(path);

    match Markup::parse(&file) {
        Some(markup) => markup.traverse(entrypoint, workdir, trail).await?,
        _ => (),
    }

    Ok(())
}
