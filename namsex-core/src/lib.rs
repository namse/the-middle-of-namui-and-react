mod button;
mod me;
mod me1;
mod text;

pub use button::*;
pub use me::*;
pub use me1::*;
pub use once_cell::sync::Lazy;
pub use std::{any::Any, collections::HashMap, fmt::Debug, sync::Mutex};
pub use text::*;

static LOG_FN: once_cell::sync::OnceCell<Mutex<Box<dyn Fn(&str) + Send + Sync>>> =
    once_cell::sync::OnceCell::new();

fn log(msg: &str) {
    (LOG_FN.get().unwrap().lock().unwrap())(msg);
}

macro_rules! log {
    ($($arg:tt)*) => (log(&format!($($arg)*)));
}

pub fn run<Root: Component<Props = Props> + 'static, Props: Any>(
    props: Props,
    log_fn: impl Fn(&str) + Send + Sync + 'static,
    sync_tree_to_platform: impl Fn(&TreeNode),
) {
    LOG_FN.get_or_init(|| Mutex::new(Box::new(log_fn)));

    let root_component = Root::create(&props);
    let tree = resolve_tree(Box::new(root_component), &props);

    log!("{:#?}", tree);

    (sync_tree_to_platform)(&tree);
}

#[derive(Debug)]
pub enum TreeNode {
    Component {
        component: Box<dyn InternalComponent>,
        children: Vec<TreeNode>,
    },
    PlatformNode {
        platform_node: PlatformNode,
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
        RenderingTree::Node(platform_node) => TreeNode::PlatformNode {
            platform_node: platform_node,
        },
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

pub trait InternalComponent: Debug {
    fn render(&mut self, props: &dyn Any) -> RenderingTree;
    fn update(&mut self, event: Box<dyn Any>);
}

pub trait Component: InternalComponent {
    type Props: Any;
    type Event: Any;
    fn create(props: &Self::Props) -> Self;
    fn render(&mut self, props: &Self::Props) -> RenderingTree;
    fn update(&mut self, event: Self::Event);
}

#[derive(Debug)]
pub struct MyRoot {
    x: i32,
}

pub struct MyRootProps {}

pub enum MyRootEvent {
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
            MyRootEvent::OnClick => log!("Clicked!"),
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
    Node(PlatformNode),
    ComponentBlueprint {
        component_type_id: std::any::TypeId,
        props: Box<dyn Any>,
    },
    //     Children(Vec<RenderingTree>),
}
// impl RenderingTree {}

#[derive(Debug)]
pub enum PlatformNode {
    Button(Button),
    Text(Text),
}
