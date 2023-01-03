use super::*;
use uuid::Uuid;

pub struct Button {
    pub id: Uuid,
    pub text: String,
    pub on_click_callback: OnClickCallback,
}

impl std::fmt::Debug for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Button")
            .field("id", &self.id)
            .field("text", &self.text)
            .finish()
    }
}

type OnClickCallback = Box<dyn Fn(ButtonClickEvent) -> Option<Box<dyn Any>>>;

impl Button {
    pub fn render<YourEvent: Any>(
        text: impl AsRef<str>,
        on_click: impl Fn(ButtonClickEvent) -> Option<YourEvent> + 'static,
    ) -> RenderingTree {
        let node_id = Uuid::new_v4();
        {
            let owner_id: Uuid = RENDERING_COMPONENT_ID
                .get()
                .unwrap()
                .lock()
                .unwrap()
                .clone();

            crate::log!("owner_id: {}", owner_id);
            PLATFORM_NODE_OWNER_ID_MAP
                .lock()
                .unwrap()
                .insert(node_id, owner_id);
        }

        RenderingTree::Node(PlatformNode::Button(Button {
            id: node_id,
            text: text.as_ref().to_string(),
            on_click_callback: Box::new(move |event: ButtonClickEvent| {
                on_click(event).map(|event| Box::new(event) as Box<dyn Any>)
            }),
        }))
    }
    pub fn on_event(&mut self, event: ButtonEvent) -> Option<Box<dyn Any>> {
        match event {
            ButtonEvent::Click => (self.on_click_callback)(ButtonClickEvent {}),
        }
    }
}

#[derive(Debug)]
pub struct ButtonClickEvent {}

#[derive(Debug)]
pub enum ButtonEvent {
    Click,
}
