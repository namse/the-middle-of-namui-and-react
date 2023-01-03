use super::*;
use uuid::Uuid;

#[derive(Debug)]
pub struct Text {
    pub id: Uuid,
    pub(crate) text: String,
}

impl Text {
    pub fn render(text: impl AsRef<str>) -> RenderingTree {
        RenderingTree::Node(PlatformNode::Text(Text {
            id: Uuid::new_v4(),
            text: text.as_ref().to_string(),
        }))
    }
}
