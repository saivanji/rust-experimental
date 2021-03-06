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
}
