use super::*;

#[derive(Debug)]
pub struct Me1 {
    pub x: i32,
}

pub struct Me1Props {}

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
        println!("Me1::render x={}", self.x);
        Button::render("I am button!", |event| {
            println!("Button clicked! {:?}", event);
            Some(Me1Event::OnClick)
        })
    }

    fn update(&mut self, event: Self::Event) {
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
            let mut generators = COMPONENT_GENERATORS.lock().unwrap();
            if !generators.contains_key(&component_type_id) {
                generators.insert(
                    component_type_id,
                    Box::new(|props| {
                        let component: Me1 =
                            Component::create(props.downcast_ref::<Me1Props>().unwrap());
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

impl InternalComponent for Me1 {
    fn render(&mut self, props: &dyn Any) -> RenderingTree {
        Component::render(self, props.downcast_ref::<Me1Props>().unwrap())
    }

    fn update(&mut self, event: Box<dyn Any>) {
        Component::update(self, *event.downcast::<Me1Event>().unwrap())
    }
}
