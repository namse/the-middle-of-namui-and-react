use namsex_core::*;

fn main() {
    run::<MyRoot, _>(
        MyRootProps {},
        |str| println!("{}", str),
        sync_tree_to_platform,
    );
}

fn sync_tree_to_platform(tree: &TreeNode) {
    match tree {
        TreeNode::Component {
            component: _,
            children,
        } => {
            for child in children {
                sync_tree_to_platform(child);
            }
        }
        TreeNode::PlatformNode { platform_node } => match platform_node {
            PlatformNode::Button(button) => {
                println!("Attach button node: {:?}", button);
            }
            PlatformNode::Text(text) => {
                println!("Attach text node: {:?}", text);
            }
        },
    }
}
