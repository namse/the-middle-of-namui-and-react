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
use std::{any::TypeId, sync::Arc};
use uuid::Uuid;

pub async fn run<Root: Component<Props = Props> + 'static, Props: Any>(
    props: Props,
    log_fn: impl Fn(&str) + Send + Sync + 'static,
    sync_tree_to_platform: impl Fn(&mut TreeNode),
) {
    let mut event_receiver = event::init();
    LOG_FN.get_or_init(|| Mutex::new(Box::new(log_fn)));

    let root_component = ComponentWrapper::new(Box::new(Root::create(&props)));
    let mut tree_node = reconciliation(root_component, Box::new(props), None, true);

    loop {
        (sync_tree_to_platform)(&mut tree_node);

        log!("{:#?}", tree_node);
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

            let Some(event_to) = platform_node.on_event(event) else {
                return tree_node;
            };

            tree_node = tree_node
                .edit_component_node(event_to.instance_id, move |mut owner| {
                    let TreeNode::Component { component, .. } = &mut owner else {
                        unreachable!("owner of platform node must be a component");
                    };
                    component.internal.update(event_to.event.as_ref());
                    top_reconciliation(owner)
                })
                .unwrap();
        }
    }
    tree_node
}

fn should_keep_to_prev_tree_node(
    component: &ComponentWrapper,
    props: &Box<dyn Any>,
    prev_tree_node: &Option<TreeNode>,
    force_render: bool,
) -> bool {
    if force_render {
        return false;
    }

    let Some(prev_tree_node) = &prev_tree_node else {
        return false;
    };

    let TreeNode::Component {
        component: prev_component,
        props: prev_props,
        children: _,
    } = &prev_tree_node else {
        return false;
    };

    let component_type_id = component.component_type_id();

    component_type_id == prev_component.component_type_id()
        && boxed_props_eq(component_type_id, &prev_props, &props)
}

fn top_reconciliation(tree_node: TreeNode) -> TreeNode {
    let TreeNode::Component { mut component, props, mut children } = tree_node else {
        unreachable!("owner of platform node must be a component");
    };

    let rendering_tree = component.render(props.as_ref());
    let prev_child_tree_node = children.pop_front();

    let child = match rendering_tree {
        RenderingTree::ComponentBlueprint {
            component_type_id,
            props,
        } => {
            let child_component = create_component(component_type_id, props.as_ref());
            reconciliation(child_component, props, prev_child_tree_node, false)
        }
        RenderingTree::Node(platform_node) => match prev_child_tree_node {
            Some(TreeNode::PlatformNode {
                platform_node: prev_platform_node,
                prev_platform_node: _,
                rendered_real_dom,
            }) => TreeNode::PlatformNode {
                platform_node,
                prev_platform_node: Some(prev_platform_node),
                rendered_real_dom,
            },
            _ => TreeNode::PlatformNode {
                platform_node,
                prev_platform_node: None,
                rendered_real_dom: None,
            },
        },
    };

    TreeNode::Component {
        component,
        props,
        children: vec![child].into(),
    }
}

fn reconciliation(
    mut component: ComponentWrapper,
    props: Box<dyn Any>,
    prev_tree_node: Option<TreeNode>,
    force_render: bool,
) -> TreeNode {
    if should_keep_to_prev_tree_node(&component, &props, &prev_tree_node, force_render) {
        return prev_tree_node.unwrap();
    }

    let rendering_tree = component.render(props.as_ref());
    let prev_child_tree_node = prev_tree_node.and_then(get_child_tree_node);

    let child = match rendering_tree {
        RenderingTree::ComponentBlueprint {
            component_type_id,
            props,
        } => {
            let child_component = create_component(component_type_id, props.as_ref());
            reconciliation(child_component, props, prev_child_tree_node, false)
        }
        RenderingTree::Node(platform_node) => match prev_child_tree_node {
            Some(TreeNode::PlatformNode {
                platform_node: prev_platform_node,
                prev_platform_node: _,
                rendered_real_dom,
            }) => TreeNode::PlatformNode {
                platform_node,
                prev_platform_node: Some(prev_platform_node),
                rendered_real_dom,
            },
            _ => TreeNode::PlatformNode {
                platform_node,
                prev_platform_node: None,
                rendered_real_dom: None,
            },
        },
    };

    TreeNode::Component {
        component,
        props,
        children: vec![child].into(),
    }
}

fn get_child_tree_node(tree_node: TreeNode) -> Option<TreeNode> {
    log!("hi");
    log!("get_child_tree_node: {:#?}", tree_node);
    let TreeNode::Component {
            component: _,
            props: _,
            mut children,
        } = tree_node else {
            return None
        };

    children.pop_front()
}

fn create_component(component_type_id: std::any::TypeId, props: &dyn Any) -> ComponentWrapper {
    COMPONENT_GENERATOR_MAP
        .lock()
        .unwrap()
        .get(&component_type_id)
        .unwrap()(props)
}

type GeneratorMap =
    HashMap<std::any::TypeId, Box<dyn Fn(&dyn Any) -> ComponentWrapper + Send + Sync>>;
static COMPONENT_GENERATOR_MAP: Lazy<Mutex<GeneratorMap>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
type PropsEqMap = HashMap<std::any::TypeId, Box<dyn Fn(&dyn Any, &dyn Any) -> bool + Send + Sync>>;
static PROPS_EQ_MAP: Lazy<Mutex<PropsEqMap>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub trait InternalComponent: Debug {
    fn render(&mut self, props: &dyn Any) -> RenderingTree;
    fn update(&mut self, event: &dyn Any);
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
    pub(crate) fn component_type_id(&self) -> std::any::TypeId {
        self.internal.component_type_id()
    }
}

#[derive(Clone)]
pub struct EventTo {
    instance_id: Uuid,
    event: Arc<dyn Any>,
}

impl PartialEq for EventTo {
    fn eq(&self, other: &Self) -> bool {
        self.instance_id == other.instance_id && Arc::ptr_eq(&self.event, &other.event)
    }
}

pub trait Component: InternalComponent {
    type Props: Any;
    type Event: Any;
    fn create(props: &Self::Props) -> Self;
    fn render(&mut self, props: &Self::Props) -> RenderingTree;
    fn update(&mut self, event: &Self::Event);
    fn event_handler(&self, event: impl Any) -> EventTo {
        let rendering_component_id = *RENDERING_COMPONENT_ID.get().unwrap().lock().unwrap();
        EventTo {
            instance_id: rendering_component_id,
            event: Arc::new(event),
        }
    }
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

    fn update(&mut self, event: &Self::Event) {
        match event {
            MyRootEvent::OnClick => log!("Clicked!"),
        }
    }
}

impl InternalComponent for MyRoot {
    fn render(&mut self, props: &dyn Any) -> RenderingTree {
        Component::render(self, props.downcast_ref::<MyRootProps>().unwrap())
    }

    fn update(&mut self, event: &dyn Any) {
        Component::update(self, event.downcast_ref::<MyRootEvent>().unwrap())
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

fn boxed_props_eq(
    component_type_id: TypeId,
    props_a: &Box<dyn Any>,
    props_b: &Box<dyn Any>,
) -> bool {
    props_a.type_id() == props_b.type_id()
        && PROPS_EQ_MAP
            .lock()
            .unwrap()
            .get(&component_type_id)
            .unwrap()(props_a.as_ref(), props_b.as_ref())
}
