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
        prev_platform_node: Option<PlatformNode>,
        rendered_real_dom: Option<Box<dyn Any>>,
    },
}

impl TreeNode {
    pub(crate) fn edit_component_node(
        self,
        component_id: Uuid,
        callback: impl FnOnce(TreeNode) -> TreeNode + 'static,
    ) -> Result<TreeNode, TreeNode> {
        // TODO: Optimize this

        let is_me = {
            match &self {
                TreeNode::Component {
                    component,
                    props: _,
                    children: _,
                } => component.id == component_id,
                TreeNode::PlatformNode { .. } => false,
            }
        };

        if is_me {
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
                        if child.has_owner_of_component_node_recursively(component_id) {
                            return Ok(TreeNode::Component {
                                component,
                                props,
                                children: new_children
                                    .into_iter()
                                    .chain(vec![child
                                        .edit_component_node(component_id, callback)
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
                TreeNode::PlatformNode { .. } => Result::Err(self),
            }
        }
    }
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
            TreeNode::PlatformNode {
                platform_node,
                rendered_real_dom: _,
                prev_platform_node: _,
            } => {
                if platform_node.id() == node_id {
                    Some(platform_node)
                } else {
                    None
                }
            }
        }
    }

    fn has_owner_of_component_node_recursively(&self, component_id: Uuid) -> bool {
        match self {
            TreeNode::Component {
                component,
                props: _,
                children,
            } => {
                if component.id == component_id {
                    true
                } else {
                    for child in children {
                        if child.has_owner_of_component_node_recursively(component_id) {
                            return true;
                        }
                    }
                    false
                }
            }
            TreeNode::PlatformNode { .. } => false,
        }
    }
}

pub static PLATFORM_NODE_OWNER_ID_MAP: Lazy<Mutex<HashMap<Uuid, Uuid>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
