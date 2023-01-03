mod button;
pub mod event;
mod me;
mod me1;
mod platform_node;
mod text;
mod tree_node;

pub use button::*;
pub use me::*;
pub use me1::*;
pub use once_cell::sync::Lazy;
pub use platform_node::*;
pub use std::{any::Any, collections::HashMap, fmt::Debug, sync::Mutex};
pub use text::*;
pub use tree_node::*;

static LOG_FN: once_cell::sync::OnceCell<Mutex<Box<dyn Fn(&str) + Send + Sync>>> =
    once_cell::sync::OnceCell::new();

fn log_fn(msg: &str) {
    (LOG_FN.get().unwrap().lock().unwrap())(msg);
}

macro_rules! log {
    ($($arg:tt)*) => (log_fn(&format!($($arg)*)));
}

pub(crate) use log;
use uuid::Uuid;

pub async fn run<Root: Component<Props = Props> + 'static, Props: Any>(
    props: Props,
    log_fn: impl Fn(&str) + Send + Sync + 'static,
    sync_tree_to_platform: impl Fn(&TreeNode),
) {
    let mut event_receiver = event::init();
    LOG_FN.get_or_init(|| Mutex::new(Box::new(log_fn)));

    let root_component = ComponentWrapper::new(Box::new(Root::create(&props)));
    let mut tree_node = resolve_tree(root_component, Box::new(props));

    loop {
        log!("{:#?}", tree_node);
        (sync_tree_to_platform)(&tree_node);

        let event = event_receiver.recv().await.unwrap();

        tree_node = handle_event(event, tree_node);
    }
}

static RENDERING_COMPONENT_ID: once_cell::sync::OnceCell<Mutex<Uuid>> =
    once_cell::sync::OnceCell::new();

fn handle_event(event: event::Event, mut tree_node: TreeNode) -> TreeNode {
    match event.downcast::<event::NamsexEvent>().unwrap() {
        event::NamsexEvent::NodeEvent { node_id, event } => {
            let Some(platform_node) = tree_node.find_platform_node(node_id) else {
                return tree_node;
            };

            let Some(event_for_owner) = platform_node.on_event(event) else {
                return tree_node;
            };

            tree_node = tree_node
                .edit_owner_of_platform_node(node_id, |owner| {
                    let TreeNode::Component { mut component, props, children: _ } = owner else {
                    unreachable!("owner of platform node must be a component");
                };
                    component.internal.update(event_for_owner);
                    resolve_tree(component, props)
                })
                .unwrap();
        }
    }
    tree_node
}

fn resolve_tree(mut component: ComponentWrapper, props: Box<dyn Any>) -> TreeNode {
    let rendering_tree = component.render(props.as_ref());
    let child = match rendering_tree {
        RenderingTree::ComponentBlueprint {
            component_type_id,
            props,
        } => {
            let child_component = create_component(component_type_id, props.as_ref());
            resolve_tree(child_component, props)
        }
        RenderingTree::Node(platform_node) => TreeNode::PlatformNode { platform_node },
    };

    let address = &component as *const _ as usize;
    log!("rendering &component address after: {:#?}", address);
    TreeNode::Component {
        component,
        props,
        children: vec![child].into(),
    }
}

fn create_component(component_type_id: std::any::TypeId, props: &dyn Any) -> ComponentWrapper {
    COMPONENT_GENERATORS
        .lock()
        .unwrap()
        .get(&component_type_id)
        .unwrap()(props)
}

type GeneratorMap =
    HashMap<std::any::TypeId, Box<dyn Fn(&dyn Any) -> ComponentWrapper + Send + Sync>>;
static COMPONENT_GENERATORS: Lazy<Mutex<GeneratorMap>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub trait InternalComponent: Debug {
    fn render(&mut self, props: &dyn Any) -> RenderingTree;
    fn update(&mut self, event: Box<dyn Any>);
    fn component_type_id(&self) -> std::any::TypeId;
}

#[derive(Debug)]
pub struct ComponentWrapper {
    pub(crate) id: Uuid,
    pub(crate) internal: Box<dyn InternalComponent>,
}

impl ComponentWrapper {
    pub(crate) fn new(internal: Box<dyn InternalComponent>) -> Self {
        ComponentWrapper {
            id: Uuid::new_v4(),
            internal,
        }
    }
    pub(crate) fn render(&mut self, props: &dyn Any) -> RenderingTree {
        let id = self.id;
        *RENDERING_COMPONENT_ID
            .get_or_init(|| Mutex::new(id))
            .lock()
            .unwrap() = id;

        self.internal.render(props)
    }
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

    fn component_type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<Self>()
    }
}

#[derive(Debug)]
pub enum RenderingTree {
    Node(PlatformNode),
    ComponentBlueprint {
        component_type_id: std::any::TypeId,
        props: Box<dyn Any>,
    },
}
