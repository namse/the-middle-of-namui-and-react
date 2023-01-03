use super::*;
use std::collections::VecDeque;
use uuid::Uuid;

#[derive(Debug)]
pub enum TreeNode {
    Component {
        component: ComponentWrapper,
        props: Box<dyn Any>,
        children: VecDeque<TreeNode>,
    },
    PlatformNode {
        platform_node: PlatformNode,
    },
}

impl TreeNode {
    pub(crate) fn find_platform_node(&mut self, node_id: Uuid) -> Option<&mut PlatformNode> {
        match self {
            TreeNode::Component {
                component: _,
                props: _,
                children,
            } => {
                for child in children {
                    if let Some(platform_node) = child.find_platform_node(node_id) {
                        return Some(platform_node);
                    }
                }
                None
            }
            TreeNode::PlatformNode { platform_node } => {
                if platform_node.id() == node_id {
                    Some(platform_node)
                } else {
                    None
                }
            }
        }
    }
    pub(crate) fn edit_owner_of_platform_node(
        self,
        node_id: Uuid,
        callback: impl FnOnce(TreeNode) -> TreeNode + 'static,
    ) -> Result<TreeNode, TreeNode> {
        // TODO: Optimize this

        let is_me_owner = {
            match &self {
                TreeNode::Component {
                    component,
                    props: _,
                    children: _,
                } => {
                    let owner_id = {
                        let map = PLATFORM_NODE_OWNER_ID_MAP.lock().unwrap();
                        map.get(&node_id).unwrap().clone()
                    };

                    let component_id = component.id;
                    component_id == owner_id
                }
                TreeNode::PlatformNode { platform_node: _ } => false,
            }
        };

        crate::log!("is_me_owner: {}", is_me_owner);

        if is_me_owner {
            Ok(callback(self))
        } else {
            match self {
                TreeNode::Component {
                    component,
                    props,
                    mut children,
                } => {
                    let mut new_children = vec![];
                    while let Some(child) = children.pop_front() {
                        if child.has_owner_of_platform_node_recursively(node_id) {
                            return Ok(TreeNode::Component {
                                component,
                                props,
                                children: new_children
                                    .into_iter()
                                    .chain(vec![child
                                        .edit_owner_of_platform_node(node_id, callback)
                                        .unwrap()])
                                    .chain(children)
                                    .collect(),
                            });
                        }

                        new_children.push(child);
                    }
                    Result::Err(TreeNode::Component {
                        component,
                        props,
                        children: new_children.into(),
                    })
                }
                TreeNode::PlatformNode { platform_node } => {
                    Result::Err(TreeNode::PlatformNode { platform_node })
                }
            }
        }
    }

    fn has_owner_of_platform_node_recursively(&self, node_id: Uuid) -> bool {
        match self {
            TreeNode::Component {
                component,
                props: _,
                children,
            } => {
                let owner_id = {
                    let map = PLATFORM_NODE_OWNER_ID_MAP.lock().unwrap();
                    map.get(&node_id).unwrap().clone()
                };

                let component_id = component.id;

                if component_id == owner_id {
                    true
                } else {
                    for child in children {
                        if child.has_owner_of_platform_node_recursively(node_id) {
                            return true;
                        }
                    }
                    false
                }
            }
            TreeNode::PlatformNode { platform_node: _ } => false,
        }
    }
}

pub static PLATFORM_NODE_OWNER_ID_MAP: Lazy<Mutex<HashMap<Uuid, Uuid>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
