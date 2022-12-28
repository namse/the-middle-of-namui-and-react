use std::any::Any;

fn main() {
    let my_comp = MyComp { x: 0 };
    let system = System {};

    system.run(my_comp, MyCompProps {});
}

struct System {}
impl System {
    fn run<Props: Any>(&self, component: impl Component, props: Props) {
        let tree = component.render(Box::new(props));
        // 새로 그려야하는지, 아닌지 판단한다.

        // 전체 설계로를 처음에 완성한다.
        // 그리고 그걸로 렌더링을 한다.
        // 만약 업데이트가 호출된 컴포넌트가 있으면, 해당 컴포넌트 아래로 다시 렌더링을 한다.
        // 근데 그건 어려우니까, 뭔가 업데이트가 있으면 전체 렌더링을 다시 한다. 근데, 변경사항이 없는 애는 렌더링을 하지 않는다.

        match tree {
            RenderingTree::Node(node) => match node {
                RenderingNode::Button(button) => {
                    let should_redraw = is_different_element || is_props_different;
                    if should_redraw {
                        let button_element = create_button_element();
                        let button_element_id = button_element.id();
                        dom_parent.append_child(button_element);
                        button_element
                            .set_on_click(|event| call_button_on_click(button_element_id, event));
                    }

                    if let Some(click_callback) = &button.click_callback {
                        let button_element_id = get_button_element_id();
                        coonnect_button_on_click(button_element_id, click_callback);
                    }
                }
            },
            RenderingTree::Children(_) => todo!(),
            RenderingTree::Component(_) => todo!(),
        }
    }
}

trait Component {
    fn render(&self, props: Box<dyn Any>) -> RenderingTree;
}

struct MyComp {
    x: i32,
}

struct MyCompProps {}

enum MyCompEvent {
    OnClick,
}

impl MyComp {
    fn render(&self, props: MyCompProps) -> RenderingTree {
        Button::build()
            .on_click(|event| Some(MyCompEvent::OnClick))
            .done()
    }
    fn update(&mut self, event: MyCompEvent) {
        match event {
            MyCompEvent::OnClick => println!("Clicked!"),
        }
    }
}

impl Component for MyComp {
    fn render(&self, props: Box<dyn Any>) -> RenderingTree {
        MyComp::render(&self, *props.downcast().unwrap())
    }
}

enum RenderingTree {
    Node(RenderingNode),
    Component(Box<dyn Component>),
    Children(Vec<RenderingTree>),
}
impl RenderingTree {}

enum RenderingNode {
    Button(Button),
}

struct Button {
    click_callback: Option<Box<dyn Any>>,
}
impl Button {
    fn build() -> Self {
        Button {
            click_callback: None,
        }
    }
    fn on_click<T>(mut self, callback: impl Fn(ButtonClickEvent) -> Option<T> + 'static) -> Self
    where
        T: Any,
    {
        self.click_callback = Some(Box::new(callback));
        self
    }

    fn done(self) -> RenderingTree {
        RenderingTree::Node(RenderingNode::Button(self))
    }
}

struct ButtonClickEvent {}
