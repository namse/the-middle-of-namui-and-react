use super::*;

#[derive(Debug)]
pub struct Text {
    text: String,
}

impl Text {
    pub fn render(text: impl AsRef<str>) -> RenderingTree {
        RenderingTree::Node(RenderingNode::Text(Text {
            text: text.as_ref().to_string(),
        }))
    }
}
