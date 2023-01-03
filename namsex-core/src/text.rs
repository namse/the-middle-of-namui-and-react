use super::*;

#[derive(Debug)]
pub struct Text {
    pub(crate) text: String,
}

impl Text {
    pub fn render(text: impl AsRef<str>) -> RenderingTree {
        RenderingTree::Node(PlatformNode::Text(Text {
            text: text.as_ref().to_string(),
        }))
    }
}
