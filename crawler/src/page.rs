use crate::{Entrypoint, File, Location, Markup, Trail};
use anyhow::Result;

pub async fn process(
    path: &str,
    entrypoint: &Entrypoint,
    workdir: &Location,
    trail: &Trail,
) -> Result<()> {
    let location = workdir.concat(path);

    let bytes = entrypoint.link(path).fetch().await?;
    let file = File::from(bytes, &location);

    file.persist()?;

    Ok(match Markup::parse(&file) {
        Some(markup) => markup.traverse(&trail, &entrypoint).await?,
        _ => (),
    })
}
