use crate::{page, Entrypoint, File, Location, Node, NodeKind, Trail};
use anyhow::Result;
use async_std::task;
use scraper::{Html, Selector};

pub struct Markup {
    html: Html,
}

impl Markup {
    pub fn parse(file: &File) -> Option<Self> {
        file.text().and_then(|text| {
            let html = Html::parse_document(text);

            if html.errors.len() != 0 {
                return None;
            }

            Some(Self { html })
        })
    }

    pub fn traverse(
        &self,
        entrypoint: &Entrypoint,
        workdir: &Location,
        trail: &mut Trail,
    ) -> Result<()> {
        let mut all = Vec::new();

        let mut anchors = self.select(NodeKind::Anchor);
        let mut links = self.select(NodeKind::Link);
        let mut scripts = self.select(NodeKind::Script);

        all.append(&mut anchors);
        all.append(&mut links);
        all.append(&mut scripts);

        for node in all {
            match node.href() {
                Some(path) if !trail.has(&path) => {
                    task::spawn(page::process(&path, entrypoint, workdir, trail));
                    // page::process(&path, entrypoint, workdir, trail).await?
                }
                _ => continue,
            }
        }

        Ok(())
    }

    fn select(&self, kind: NodeKind) -> Vec<Node> {
        let selector = match kind {
            NodeKind::Anchor => "a",
            NodeKind::Link => "link",
            NodeKind::Script => "script",
        };

        self.html
            .select(&Selector::parse(selector).unwrap())
            .map(|element| Node::new(kind, element))
            .collect()
    }
}
