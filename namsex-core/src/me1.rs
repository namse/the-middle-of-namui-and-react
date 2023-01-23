use super::*;

#[derive(Debug)]
pub struct Me1 {
    pub x: i32,
}

#[derive(PartialEq)]
pub struct Me1Props {
    pub on_button_click: EventHandler,
    pub value: i32,
}

pub enum Me1Event {
    OnClick,
}

impl Component for Me1 {
    type Props = Me1Props;
    type Event = Me1Event;

    fn create(props: &Self::Props) -> Self {
        Me1 { x: 0 }
    }

    fn render(&mut self, props: &Self::Props) -> RenderingTree {
        Button::render(
            format!(
                "I am button! self.x: {}, props.value: {}",
                self.x, props.value
            ),
            &props.on_button_click,
        )
    }

    fn update(&mut self, event: &Self::Event) {
        match event {
            Me1Event::OnClick => {
                println!("Me1Event::OnClick event!");
                self.x += 1;
            }
        }
    }
}

impl Me1 {
    pub fn render(props: Me1Props) -> RenderingTree {
        let component_type_id = std::any::TypeId::of::<Self>();
        {
            let mut generators = COMPONENT_GENERATOR_MAP.lock().unwrap();
            if !generators.contains_key(&component_type_id) {
                generators.insert(
                    component_type_id,
                    Box::new(|props| {
                        let props = props.downcast_ref::<Me1Props>().unwrap();
                        let component: Me1 = Component::create(props);
                        ComponentWrapper::new(Box::new(component))
                    }),
                );
            }
        }
        {
            let mut props_eq_map = PROPS_EQ_MAP.lock().unwrap();
            if !props_eq_map.contains_key(&component_type_id) {
                props_eq_map.insert(
                    component_type_id,
                    Box::new(|a, b| {
                        let Some(a) = a.downcast_ref::<Me1Props>() else {
                            return false;
                        };
                        let Some(b) = b.downcast_ref::<Me1Props>() else {
                            return false;
                        };
                        a == b
                    }),
                );
            }
        }

        RenderingTree::ComponentBlueprint {
            component_type_id,
            props: Box::new(props),
        }
    }
}

impl InternalComponent for Me1 {
    fn render(&mut self, props: &dyn Any) -> RenderingTree {
        Component::render(self, props.downcast_ref::<Me1Props>().unwrap())
    }

    fn update(&mut self, event: &dyn Any) {
        Component::update(self, event.downcast_ref::<Me1Event>().unwrap())
    }

    fn component_type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<Self>()
    }
}
