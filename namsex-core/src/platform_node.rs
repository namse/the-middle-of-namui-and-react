use super::*;
use uuid::Uuid;

#[derive(Debug)]
pub enum PlatformNode {
    Button(Button),
    Text(Text),
}
impl PlatformNode {
    pub(crate) fn on_event(&mut self, event: event::Event) -> Option<EventTo> {
        match self {
            PlatformNode::Button(button) => button.on_event(event.downcast().unwrap()),
            PlatformNode::Text(text) => unreachable!(),
        }
    }

    pub(crate) fn id(&self) -> Uuid {
        match self {
            PlatformNode::Button(button) => button.id,
            PlatformNode::Text(text) => text.id,
        }
    }
}
