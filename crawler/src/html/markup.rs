use crate::{page, Entrypoint, File, Location, Node, NodeKind, Trail};
use anyhow::Result;
use futures::future::join_all;
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

    pub async fn traverse(
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

        let f = join_all(
            all.iter()
                .filter(|node| node.href().map(|path| trail.has(&path)).unwrap_or(false))
                .map(|node| {
                    let path = node.href().unwrap();

                    page::process(&path, entrypoint, workdir, trail)
                }),
        );

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
