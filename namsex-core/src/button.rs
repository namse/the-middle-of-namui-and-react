use super::*;
use uuid::Uuid;

pub struct Button {
    pub id: Uuid,
    pub text: String,
    pub on_click: EventTo,
}

impl std::fmt::Debug for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Button")
            .field("id", &self.id)
            .field("text", &self.text)
            .finish()
    }
}

impl Button {
    pub fn render(text: impl AsRef<str>, on_click: &EventTo) -> RenderingTree {
        println!("Button::render");
        let node_id = Uuid::new_v4();
        {
            let owner_id: Uuid = RENDERING_COMPONENT_ID
                .get()
                .unwrap()
                .lock()
                .unwrap()
                .clone();

            PLATFORM_NODE_OWNER_ID_MAP
                .lock()
                .unwrap()
                .insert(node_id, owner_id);
        }

        RenderingTree::Node(PlatformNode::Button(Button {
            id: node_id,
            text: text.as_ref().to_string(),
            on_click: on_click.clone(),
        }))
    }
    pub fn on_event(&mut self, event: ButtonEvent) -> Option<EventTo> {
        match event {
            ButtonEvent::Click => Some(self.on_click.clone()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ButtonClickEvent {}

#[derive(Debug)]
pub enum ButtonEvent {
    Click,
}
