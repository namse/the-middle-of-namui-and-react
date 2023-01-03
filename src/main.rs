mod me;
mod me1;
mod text;

use me::*;
use me1::*;
use once_cell::sync::Lazy;
use std::{any::Any, collections::HashMap, fmt::Debug, sync::Mutex};
use text::*;

fn main() {
    run::<MyRoot, _>(MyRootProps {});
}

fn run<Root: Component<Props = Props> + 'static, Props: Any>(props: Props) {
    let root_component = Root::create(&props);
    let tree = resolve_tree(Box::new(root_component), &props);

    println!("{:#?}", tree);
}

#[derive(Debug)]
enum TreeNode {
    Component {
        component: Box<dyn InternalComponent>,
        children: Vec<TreeNode>,
    },
    EndNode {
        rendering_node: RenderingNode,
    },
}

fn resolve_tree(mut component: Box<dyn InternalComponent>, props: &dyn Any) -> TreeNode {
    let rendering_tree = component.render(props);

    match rendering_tree {
        RenderingTree::ComponentBlueprint {
            component_type_id,
            props,
        } => {
            let child = create_component(component_type_id, props.as_ref());
            let child_tree_node = resolve_tree(child, props.as_ref());
            TreeNode::Component {
                component,
                children: vec![child_tree_node],
            }
        }
        RenderingTree::Node(rendering_node) => TreeNode::EndNode { rendering_node },
    }
}

fn create_component(
    component_type_id: std::any::TypeId,
    props: &dyn Any,
) -> Box<dyn InternalComponent> {
    COMPONENT_GENERATORS
        .lock()
        .unwrap()
        .get(&component_type_id)
        .unwrap()(props)
}

type GeneratorMap =
    HashMap<std::any::TypeId, Box<dyn Fn(&dyn Any) -> Box<dyn InternalComponent> + Send + Sync>>;
static COMPONENT_GENERATORS: Lazy<Mutex<GeneratorMap>> = Lazy::new(|| Mutex::new(HashMap::new()));

trait InternalComponent: Debug {
    fn render(&mut self, props: &dyn Any) -> RenderingTree;
    fn update(&mut self, event: Box<dyn Any>);
}

trait Component: InternalComponent {
    type Props: Any;
    type Event: Any;
    fn create(props: &Self::Props) -> Self;
    fn render(&mut self, props: &Self::Props) -> RenderingTree;
    fn update(&mut self, event: Self::Event);
}

#[derive(Debug)]
struct MyRoot {
    x: i32,
}

struct MyRootProps {}

enum MyRootEvent {
    OnClick,
}

impl Component for MyRoot {
    type Props = MyRootProps;
    type Event = MyRootEvent;

    fn create(props: &Self::Props) -> Self {
        MyRoot { x: 0 }
    }

    fn render(&mut self, props: &Self::Props) -> RenderingTree {
        Me::render(MeProps {})
    }

    fn update(&mut self, event: Self::Event) {
        match event {
            MyRootEvent::OnClick => println!("Clicked!"),
        }
    }
}

impl InternalComponent for MyRoot {
    fn render(&mut self, props: &dyn Any) -> RenderingTree {
        Component::render(self, props.downcast_ref::<MyRootProps>().unwrap())
    }

    fn update(&mut self, event: Box<dyn Any>) {
        Component::update(self, *event.downcast::<MyRootEvent>().unwrap())
    }
}

#[derive(Debug)]
pub enum RenderingTree {
    Node(RenderingNode),
    ComponentBlueprint {
        component_type_id: std::any::TypeId,
        props: Box<dyn Any>,
    },
    //     Children(Vec<RenderingTree>),
}
// impl RenderingTree {}

#[derive(Debug)]
pub enum RenderingNode {
    // Button(Button),
    Text(Text),
}
