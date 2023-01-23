use super::*;

#[derive(Debug)]
pub struct Me {
    pub x: i32,
}

#[derive(PartialEq)]
pub struct MeProps {}

pub enum MeEvent {
    OnClick,
}

impl Component for Me {
    type Props = MeProps;
    type Event = MeEvent;

    fn create(props: &Self::Props) -> Self {
        Me { x: 0 }
    }

    fn render(&mut self, props: &Self::Props) -> RenderingTree {
        Me1::render(Me1Props {
            on_button_click: self.event_handler(MeEvent::OnClick),
            value: self.x,
        })
    }

    fn update(&mut self, event: &Self::Event) {
        match event {
            MeEvent::OnClick => {
                self.x += 1;
                println!("Clicked!");
            }
        }
    }
}

impl Me {
    pub fn render(props: MeProps) -> RenderingTree {
        let component_type_id = std::any::TypeId::of::<Self>();
        {
            let mut generators = COMPONENT_GENERATOR_MAP.lock().unwrap();
            if !generators.contains_key(&component_type_id) {
                generators.insert(
                    component_type_id,
                    Box::new(|props| {
                        let props = props.downcast_ref::<MeProps>().unwrap();
                        let component: Me = Component::create(props);
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
                        let Some(a) = a.downcast_ref::<MeProps>() else {
                            return false;
                        };
                        let Some(b) = b.downcast_ref::<MeProps>() else {
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

impl InternalComponent for Me {
    fn render(&mut self, props: &dyn Any) -> RenderingTree {
        Component::render(self, props.downcast_ref::<MeProps>().unwrap())
    }

    fn update(&mut self, event: &dyn Any) {
        Component::update(self, event.downcast_ref::<MeEvent>().unwrap())
    }

    fn component_type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<Self>()
    }
}
