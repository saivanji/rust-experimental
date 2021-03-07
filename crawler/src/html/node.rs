use scraper::ElementRef;

#[derive(Clone, Copy)]
pub enum NodeKind {
    Anchor,
    Link,
    Script,
}

pub struct Node<'a> {
    kind: &'a NodeKind,
    element: ElementRef<'a>,
}

impl<'a> Node<'a> {
    pub fn new(kind: &'a NodeKind, element: ElementRef<'a>) -> Self {
        Self { kind, element }
    }

    pub fn href(&self) -> Option<&str> {
        let value = self.element.value();

        (match self.kind {
            NodeKind::Anchor | NodeKind::Link => value.attr("href"),
            NodeKind::Script => value.attr("src"),
        })
        // filtering out external links
        .filter(|href| href.starts_with("/"))
    }
}
