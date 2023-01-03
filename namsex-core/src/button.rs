use super::*;

#[derive(Debug)]
pub struct Button {
    pub text: String,
    pub on_click_callback: Box<dyn Any>,
}

impl Button {
    pub fn render<YourEvent>(
        text: impl AsRef<str>,
        on_click: impl Fn(ButtonClickEvent) -> Option<YourEvent> + 'static,
    ) -> RenderingTree {
        RenderingTree::Node(PlatformNode::Button(Button {
            text: text.as_ref().to_string(),
            on_click_callback: Box::new(on_click),
        }))
    }
}

#[derive(Debug)]
pub struct ButtonClickEvent {}
