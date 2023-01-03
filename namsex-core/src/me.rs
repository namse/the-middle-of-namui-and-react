use super::*;

#[derive(Debug)]
pub struct Me {
    pub x: i32,
}

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
        Me1::render(Me1Props {})
    }

    fn update(&mut self, event: Self::Event) {
        match event {
            MeEvent::OnClick => println!("Clicked!"),
        }
    }
}

impl Me {
    pub fn render(props: MeProps) -> RenderingTree {
        let component_type_id = std::any::TypeId::of::<Self>();
        {
            let mut generators = COMPONENT_GENERATORS.lock().unwrap();
            if !generators.contains_key(&component_type_id) {
                generators.insert(
                    component_type_id,
                    Box::new(|props| {
                        let props = props.downcast_ref::<MeProps>().unwrap();
                        let component: Me = Component::create(props);
                        Box::new(component)
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

    fn update(&mut self, event: Box<dyn Any>) {
        Component::update(self, *event.downcast::<MeEvent>().unwrap())
    }
}
